use std::ops::Deref;
use std::str::FromStr;
use std::str::pattern::{Pattern, Searcher, SearchStep};
use itertools::{Itertools, Unique};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use stylist::yew::{Global, styled_component};
use web_sys::{EventTarget, HtmlInputElement};
use yew::{Callback, Html, html, Properties, use_state};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{html_if_some, log_display, log_str};
use crate::app::{empty_html, Route};
use crate::contexts::{ThemeContext, use_theme, TriSplit, ThemeKind};
use crate::contexts::{Table, TableLayout};
use crate::contexts::table::ExerciseMode;
use crate::contexts::{SpoilerCell, SpoilerCellProps};
use crate::contexts::exercise::ExerciseCategory::*;
use std::slice::Iter;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(PartialEq, Clone, Deserialize)]
pub struct Exercise {
    pub exercise_level: Option<String>,
    pub categories: Option<Vec<ExerciseCategory>>,
    pub info: Option<String>,
    pub title: Option<String>,
    pub path: Option<String>, // how to refer to it in the url
    pub table_layout: Option<TableLayout>,
    pub explanation: Option<String>,
    pub page: Option<i32>,
}

impl Exercise {

    pub fn effective_path(&self) -> String {
        // .map(|mut t: String| {t.remove_matches(|c: char| c.is_whitespace()); t})
        self.path.clone().or(self.title.clone()).unwrap_or("404".to_string())
    }

}


#[derive(Properties, PartialEq)]
pub struct ExerciseComponentProps {
    pub lesson_path: Option<String>,
    pub exercise: Exercise,
}

#[styled_component(ExerciseComponent)]
pub(crate) fn exercise_component(props: &ExerciseComponentProps) -> Html {
    let theme: ThemeContext = use_theme();

    let is_important = props.exercise.exercise_level.clone()
                                        .map(|s: String| ExerciseLevel::from_str(s.as_str()))
                                        .map(|r| r.ok()).flatten()
                                        .map(|l| l == ExerciseLevel::Important)
                                        .unwrap_or(false);

    let link = html_if_some(props.lesson_path.clone(), |path| {
        let exercise_link = Route::Exercise { lesson_path: path, exercise_path: props.exercise.effective_path() };
        html! {
            <div class={css!(r#"height: 20px;
                                width: 20px;
                                vertical-align: middle;
                                float: left;
                                margin-left: 8px;
                                line-height: 22px;
                                visibility: hidden;"#)}>
                <Link<Route> to={exercise_link}>
                    { "🔗" }
                </Link<Route>>
            </div>
        }
    });

    let title = html_if_some(props.exercise.title.clone(), |title| html! {
        <div class="flexer exercise-link-zone">
            <div class="filler-left">
                if is_important {
                    <div class={css!(r#"
                        height: 20px;
                        width: 20px;
                        background-size: 18px;
                        background-repeat: no-repeat;
                        background-position: center;
                        border: 0;
                        background-image: url("/assets/star.png");
                        vertical-align: middle;
                        float: right;
                        margin-right: 10px;
                    "#)}>

                    </div>
                }
            </div>
            <div class="filler-center">
                <h2 class={css!("white-space: nowrap; ")}> { title } </h2>
            </div>
            <div class="filler-right">
                { link.clone() }
            </div>
        </div>
    });

    let id_str = format!("{}-{}", props.lesson_path.clone().unwrap_or("anon".to_string()), props.exercise.title.clone().unwrap_or("anon".to_string()));
    let table_id = id_str.as_str();

    // log_display(props.exercise.table_layout.clone().unwrap().table.get(0).unwrap().get(0).unwrap());
    let info = html_if_some(props.exercise.info.clone(), |info| html! { <div class="flexer"> <p class="info">{ info } </p> </div> });
    let table = html_if_some(props.exercise.table_layout.clone(), |table_layout| html!{
        <Table key={table_id} table_layout={table_layout.clone()} theme={theme.kind.clone()} categories={props.exercise.categories.clone().unwrap_or(vec![])} id={id_str.clone()}/>
    });
    let explanation = html_if_some(props.exercise.explanation.clone(), |explanation| {
        let mut explanation_class = theme.kind.css_class_themed("");
        explanation_class.push_str(" explanation");
         html! {
             <div class="flexer">
                 <p class="info"> <Explanation text={explanation} class={explanation_class} theme={theme.kind()} /> </p>
             </div>
         }});
    let page = html_if_some(props.exercise.page.clone(), |page: i32| {
        let ref_link = format!("https://archive.org/details/A.K.WarderPali/A.%20K.%20Warder%20Pali/page/n{}/mode/1up", page + 13); // preface offset
        let hover_text = format!("Warder p. {}", page);
        html! {
            <div class="flexer">
                <a class="ref" href={ref_link} title={hover_text} target="_blank"> {"Reference"} </a>
            </div>
        }
    });

    return html! {
        <div class={css!(r#"margin-bottom: 50px;"#)}>
            { title }
            { info }
            { table }
            { page }
            { explanation }
        </div>
    }

}

#[derive(PartialEq)]
enum ExerciseLevel {
    Regular,
    Important,
}

impl FromStr for ExerciseLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Important" =>  Ok(ExerciseLevel::Important),
            _ =>            Ok(ExerciseLevel::Regular),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ExplanationProps {
    pub theme: ThemeKind,
    pub text: String,
    pub class: String,
}

pub struct Explanation {
    spoiled: bool,
}

pub enum ExplanationMsg {
    FlipState,
}

impl Component for Explanation {
    type Message = ExplanationMsg;
    type Properties = ExplanationProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { spoiled: true }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ExplanationMsg::FlipState => { self.spoiled = !self.spoiled; true },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(move |_e: MouseEvent| {
            ExplanationMsg::FlipState
        });

        let spoil_class = if self.spoiled { "spoiler_button invisible" } else { "spoiler_button visible" };
        let text = ctx.props().text.clone();
        let mut outer_class = ctx.props().theme.css_class_themed(if self.spoiled { "fade-in" } else { "table-secondary" });
        outer_class.push_str(" explanation");

        return html! {
            <div class={outer_class} onmousedown={onclick.clone()}>
                <span class={spoil_class} onmousedown={onclick}> { text } </span>
            </div>
        }
    }

}

#[derive(PartialEq, Clone, Deserialize)]
pub enum ExerciseCategory {
    Conjugation, Verbs, Vocab, Aorist, Declension
}

impl Display for ExerciseCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let to_string = match self {
                Conjugation => "conjugation",
                Verbs => "verbs",
                Vocab => "vocab",
                Aorist => "aorist",
                Declension => "declension",
        };
        f.write_str(to_string)
    }
}

impl FromStr for ExerciseCategory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "conjugation" => Ok(ExerciseCategory::Conjugation),
            "vocab" => Ok(ExerciseCategory::Vocab),
            "verbs" => Ok(ExerciseCategory::Verbs),
            "aorist" => Ok(ExerciseCategory::Aorist),
            "declension" => Ok(ExerciseCategory::Declension),
            _ => Err(()),
        }
    }
}

impl ExerciseCategory {
    pub fn to_proper_string(&self) -> String {
        match self {
            Conjugation => "Conjugations",
            Verbs => "Verbs",
            Vocab => "Vocab",
            Aorist => "Aorist",
            Declension => "Declension",
        }.to_string()
    }
}

impl ExerciseCategory {


    pub(crate) fn iterator() -> Iter<'static, ExerciseCategory> {
        static EXERCISE_CATEGORIES: [ExerciseCategory; 5] = [Conjugation, Verbs, Vocab, Aorist, Declension];
        EXERCISE_CATEGORIES.iter()
    }

}
