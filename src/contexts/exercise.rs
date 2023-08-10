use yew::{Html, html, Properties, Callback, use_state};
use stylist::yew::{Global, styled_component};
use rand::seq::SliceRandom;
use yew::prelude::*;
// use yew::html::onchange::Event;
use std::str::FromStr;
use web_sys::{EventTarget, HtmlInputElement};
use serde::{Serialize, Deserialize};
use itertools::{Unique, Itertools};

use crate::contexts::{ThemeContext, use_theme};
use std::str::pattern::{Pattern, Searcher, SearchStep};
use crate::{log_str, html_if_some, log_display};
use std::ops::Deref;
use crate::contexts::table::{ExerciseMode};
use crate::app::empty_html;
use crate::contexts::{Table, TableProps};

#[derive(PartialEq, Clone, Deserialize)]
pub struct Exercise {
    pub exercise_level: Option<String>,
    pub title: Option<String>,
    pub path: Option<String>, // how to refer to it in the url
    pub info: Option<String>,
    pub table_layout: Option<TableProps>,
    pub explanation: Option<String>,
    pub page: Option<i32>,
}

impl Exercise {

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

    let title = html_if_some(props.exercise.title.clone(), |title| html! { <h2 class={"centered"}> { title } </h2> });
    let info = html_if_some(props.exercise.info.clone(), |info| html! { <p class={"info"}> { info } </p> });
    let table = html_if_some(props.exercise.table_layout.clone(), |table_layout| { html!{ <Table ..table_layout /> } });
    let page = html_if_some(props.exercise.page.clone(), |page: i32| {
        let ref_link = format!("https://archive.org/details/A.K.WarderPali/A.%20K.%20Warder%20Pali/page/n{}/mode/1up", page + 13); // preface offset
        html! {
            <a class={"ref centered"} href={ref_link}> {"Reference"} </a>
        }
    });

    return html! {
        <div class={css!(r#""#)}>
            { title }
            { info }
            { table }
            { page }
        </div>
    }

}
