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

use crate::{html_if_some, log_display, log_str};
use crate::app::empty_html;
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

    #[allow(unused)]
    pub fn get_html(self) -> Html {
        return html! {
            <ExerciseComponent exercise={self} />
        }
    }

    pub fn effective_path(&self) -> String {
        // .map(|mut t: String| {t.remove_matches(|c: char| c.is_whitespace()); t})
        self.path.clone().or(self.title.clone()).unwrap_or("404".to_string())
    }

}

#[derive(Properties, PartialEq)]
pub struct ExerciseComponentProps {
    pub exercise: Exercise, // TODO flatten to just Exercise ?
}

#[styled_component(ExerciseComponent)]
pub(crate) fn exercise_component(props: &ExerciseComponentProps) -> Html {
    let theme: ThemeContext = use_theme();

    let title = html_if_some(props.exercise.title.clone(), |title| html! { <h2 class="centered"> { title } </h2> });
    let info = html_if_some(props.exercise.info.clone(), |info| html! { <p class="info">{ info } </p> });
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
