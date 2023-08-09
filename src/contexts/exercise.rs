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

type DataTable = Vec<Vec<String>>;

#[derive(PartialEq, Clone, Deserialize)]
pub struct Exercise {
    pub title: Option<String>,
    pub path: Option<String>, // how to refer to it in the url
    pub info: Option<String>,
    pub table: Option<DataTable>,
    pub default_mode: Option<String>,
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
    let theme: ThemeContext = use_theme();
    let mode_switcher = Callback::from(move |e: Event| {
        // let mode_str: Box<String> = Box::from(e.target_dyn_into::<HtmlInputElement>().expect("Unknown Event").value());
        // mode.set(ExerciseMode::from_str(mode_str.deref().as_str()).expect("Unknown selection"));
    });
    let mut initial_mode = props.exercise.default_mode.clone().map(|s| ExerciseMode::from_str(s.as_str()).unwrap_or(ExerciseMode::Censor)).unwrap_or(ExerciseMode::Censor);
    let mode = use_state(|| initial_mode.clone());

    let title = html_if_some(props.exercise.title.clone(), |title| html! {
            // <div class={"stretched"}>
                <h2 class={"centered"}>
                    { title }
                </h2>
            // </div>
    });

    let info = html_if_some(props.exercise.info.clone(), |info| html! {
            // <div class={"stretched"}>
                <p class={"info"}>
                    { info }
                </p>
            // </div>
    });

    // https://archive.org/details/A.K.WarderPali/A.%20K.%20Warder%20Pali/page/n{}/mode/1up
    // page

    let table = if props.exercise.table.is_some() {

        let mut extra_option = html!{};

        match initial_mode.clone() {
            ExerciseMode::DropDown(_) => {
                let mut rng = rand::thread_rng();
                let mut options: Vec<String> = props.exercise.table.clone().unwrap().iter()
                    .flat_map(|row: &Vec<String>| row.iter()
                        .map(|val: &String| split_bars(val.clone()))
                        .collect::<Vec<Option<(String, String, String)>>>())
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .map(|split: (String, String, String)| split.1)
                    .unique()
                    .collect();
                options.shuffle(&mut rng);
                log(options.get(0).unwrap());
                initial_mode = ExerciseMode::DropDown(options);
                extra_option = html! {
                    <button class={"side-options"}>{"check"}</button>
                }
            }
            _ => {}
        }

        html_if_some(props.exercise.table.clone(), |table| html! {
            <div class={css!(r#"
                display: flex;
                justify-content: center;
                margin-bottom: 50px;
            "#)}>
                <div class={"filler-left"}> </div>
                <div class={"filler-center"}>{ props.html(initial_mode.clone(), table) } </div>
                <div class={"filler-right"}>
                    <select class={"side-options"} onchange={mode_switcher.clone()}>
                        <option value="Show"> {"Reveal"} </option>
                        <option value="Censor"> {"Cover"} </option>
                        <option value="CensorByLetter"> {"Cover letter"} </option>
                        <option value="TypeField"> {"Enter text"} </option>
                        <option value="DropDown"> {"Drop down"} </option>
                    </select>
                    { extra_option.clone() }
                </div>
            </div>
        })
    } else {
        html! {}
    };

    return html! {
      <>
        <div class={css!(r#""#)}>
            { title }
            { info }
            { table }
        </div>
      </>
    }
}

fn split_bars(str: String) -> Option<(String, String, String)> {
    let left: Option<usize> = str.find("|");
    if left.is_some() {
        let (start, middle) = str.split_at(left.unwrap());
        let (_, middle) = middle.split_at(1);
        let right: Option<usize> = middle.clone().find("|");
        if right.is_some() {
            let (middle, end) = middle.split_at(right.unwrap());
            let (_, end) = end.split_at(1);
            return Some( (start.to_string(), middle.to_string(), end.to_string()) );
        }
    }
    return None;
}

impl ExerciseComponentProps {
    /**
     * generate the exercise table depending on the mode
     * unwillingly extended into multiple methods as there's no built-in table generation
     * and code isn't ran smoothly in html!{}
     */
    fn html(&self, mode: ExerciseMode, table: DataTable) -> Html {
        return html! { // return is required for some weird macro reason
            <table class={"exercise-table"}> {
                for table.iter().map(|row| { html! {
                    <tr> { for row.iter().map(|val| {
                        self.mediate(val.clone(), mode.clone())
                    }) } </tr>
                } })
            } </table>
        }
    }


    fn mediate(&self, val: String, mode: ExerciseMode) -> Html {
        let split = split_bars(val.clone());

        if split.is_some() {
            let (start, middle, end) = split.unwrap();
                return match mode {
                    ExerciseMode::Show => {
                        (html! {
                            <td> { start } <strong> { middle } </strong> { end } </td>
                        }) as Html
                    }
                    ExerciseMode::Censor => {
                        let onclick = Callback::from(move |_| {
                            // log(&*format!("{}-{}-{}", start, middle, end));
                        });
                        (html! {
                            <td class={"spoilable"} {onclick}> { start } <span class={"spoiler"}> { middle } </span> { end } </td>
                        }) as Html
                    }
                    ExerciseMode::TypeField => {
                        let onchange = Callback::from(move |e| {
                            // log(&*format!("{}-{}-{}", start, middle, end));
                        });
                        // https://jsfiddle.net/drq0nz6j/
                        (html! {
                            <td> { start } <input class={"table-input"} type="text" {onchange}/> { end } </td>
                        }) as Html
                    }
                    ExerciseMode::DropDown(options) => {
                        let onchange = Callback::from(move |e| {
                            // log(&*format!("{}-{}-{}", start, middle, end));
                        });
                        (html! {
                            <td> { start }
                                <select class={"table-input"} {onchange}>
                                    { for options.iter().map(|o| { html! {
                                        <option value={o.clone()}>{o}</option>
                                    } }) }
                                </select>
                            { end } </td>
                        }) as Html
                    }
                    _ =>  {
                        (html! {
                            <td> { val } </td>
                        }) as Html
                    }
                }
        }
        return html! { <td> {val} </td> }
    }

}



#[derive(PartialEq, Clone, Deserialize)]
pub enum ExerciseMode {
    Show, // ABC
    Censor, // [][][]
    CensorByLetter, // A[][]
    TypeField, // [Az ]
    DropDown(Vec<String>), // >ABC or >XYZ
}

impl FromStr for ExerciseMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Show" =>           Ok(ExerciseMode::Show),
            "Censor" =>         Ok(ExerciseMode::Censor),
            "CensorByLetter" => Ok(ExerciseMode::CensorByLetter),
            "TypeField" =>      Ok(ExerciseMode::TypeField),
            "DropDown"  =>      Ok(ExerciseMode::DropDown(vec![])),
            _ => Err(())
        }
    }
}

