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
use crate::contexts::{ThemeContext, use_theme, TriSplit};
use crate::contexts::{Table, TableLayout, TableHOC};
use crate::contexts::table::ExerciseMode;
use crate::contexts::{SpoilerCell, SpoilerCellProps};

#[derive(PartialEq, Clone, Deserialize)]
pub struct Exercise {
    pub exercise_level: Option<String>,
    pub info: Option<String>,
    pub title: Option<String>,
    pub path: Option<String>, // how to refer to it in the url
    pub table_layout: Option<TableLayout>,
    pub explanation: Option<String>,
    pub page: Option<i32>,
}

impl Exercise {

    pub fn effective_path(&self) -> String {
        // TODO .map(|mut t: String| {t.remove_matches(|c: char| c.is_whitespace()); t})
        self.path.clone().or(self.title.clone()).unwrap_or("404".to_string())
    }

}


#[derive(Properties, PartialEq)]
pub struct ExerciseComponentProps {
    pub lesson_path: Option<String>,
    pub exercise: Exercise, // TODO flatten to just Exercise ?
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
    let info = html_if_some(props.exercise.info.clone(), |info| html! { <div class="flexer"> <p class="info">{ info } </p> </div> });
    let table = html_if_some(props.exercise.table_layout.clone(), |table_layout| html!{ <Table theme={theme.kind.clone()} table_layout={table_layout.clone()} /> });
    let explanation = html_if_some(props.exercise.explanation.clone(), |explanation| {
        let split = TriSplit { start: "".to_string(), middle: explanation, end: "".to_string() };
        let explanation_class = theme.kind.css_class_themed("table-secondary");
         html! {
             <div class="flexer">
                 <p class="info"> <SpoilerCell text={split} class={explanation_class} do_fading={Some(())}/> </p>
             </div>
         }});
    let page = html_if_some(props.exercise.page.clone(), |page: i32| {
        let ref_link = format!("https://archive.org/details/A.K.WarderPali/A.%20K.%20Warder%20Pali/page/n{}/mode/1up", page + 13); // preface offset
        let hover_text = format!("Warder p. {}", page);
        html! {
            <a class={"ref centered"} href={ref_link} title={hover_text} target="_blank"> {"Reference"} </a>
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
