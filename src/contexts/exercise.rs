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
use crate::{log, html_if_some};
use std::ops::Deref;
use crate::contexts::table::ExerciseMode;
use crate::app::empty_html;
use crate::contexts::{Table, TableProps};

pub(crate) type DataTable = Vec<Vec<String>>;

#[derive(PartialEq, Clone, Deserialize)]
pub struct Exercise {
    pub title: Option<String>,
    pub path: Option<String>, // how to refer to it in the url
    pub info: Option<String>,
    pub table: Option<DataTable>,
    pub default_mode: Option<String>,
    pub options_style: Option<String>,
    pub exercise_level: Option<String>,
    pub explanation: Option<String>,
    pub page: Option<i32>,
}

impl Exercise {
    // pub const fn new(title: String, info: String, data: DataTable) -> Self {
    //     Self {
    //         title: Some(title),
    //         link: None,
    //         info: Some(info),
    //         table: Some(data),
    //         explanation: None,
    //         page: Some(0),
    //         default_mode: None,
    //     }
    // }

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

// pub const EXERCISES: &[Exercise] = &[Exercise::new("abc", "Yatonidānaṁ, bhikkhu, purisaṁ papañcasaññāsaṅkhā samudācaranti. Ettha ce natthi abhinanditabbaṁ abhivaditabbaṁ ajjhositabbaṁ. Esevanto rāgānusayānaṁ, esevanto paṭighānusayānaṁ, esevanto diṭṭhānusayānaṁ, esevanto vicikicchānusayānaṁ, esevanto mānānusayānaṁ, esevanto bhavarāgānusayānaṁ, esevanto avijjānusayānaṁ, esevanto daṇḍādānasatthādānakalahaviggahavivādatuvaṁtuvaṁpesuññamusāvādānaṁ. Etthete pāpakā akusalā dhammā aparisesā nirujjhantī.",
//                                                    &[&["1", "2"], &["3", "4"]])];

#[derive(Properties, PartialEq)]
pub struct ExerciseComponentProps {
    pub exercise: Exercise,
}

#[styled_component(ExerciseComponent)]
pub(crate) fn exercise_component(props: &ExerciseComponentProps) -> Html {

    let title = html_if_some(props.exercise.title.clone(), |title| html! { <h2 class={"centered"}> { title } </h2> });
    let info = html_if_some(props.exercise.info.clone(), |info| html! { <p class={"info"}> { info } </p> });
    let table = html_if_some(props.exercise.table.clone(), |table| {
        let mut initial_mode = props.exercise.default_mode.clone().map(|s| ExerciseMode::from_str(s.as_str()).unwrap_or(ExerciseMode::Censor)).unwrap_or(ExerciseMode::Censor);
        // let mode = use_state(|| initial_mode.clone());
        html!{
            <Table table={table} initial_mode={initial_mode} />
        }
    });
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
