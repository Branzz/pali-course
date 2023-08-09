use std::panic;

use stylist::yew::{ styled_component};
use yew::prelude::*;
use yew::props;
use gloo_net::http::Request;
use yew_router::prelude::*;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen::from_value;
use rand::seq::SliceRandom;

use crate::contexts::toolbar::{TOOLBAR_HEIGHT};
use crate::contexts::{RunnerProvider, ThemeKind, ThemeProvider, ThemeContext, ToolbarContext,
                      Toolbar, NamedToolbar, ExerciseComponent, ExerciseComponentProps,
                      Lessons, Lesson, Exercise, Exercises, DataTable};
use crate::{get_lessons_json, log};
use percent_encoding::percent_decode_str;
use yew::{Html, html, Properties, Callback, use_state};
use yew::prelude::*;
// use yew::html::onchange::Event;
use std::str::FromStr;
use web_sys::{EventTarget, HtmlInputElement};
use serde::{Serialize, Deserialize};
use itertools::{Unique, Itertools};

use crate::contexts::{use_theme};
use std::str::pattern::{Pattern, Searcher, SearchStep};
use crate::{html_if_some};
use std::ops::Deref;
use std::slice::Iter;
use crate::app::empty_html;
use crate::contexts::table::OptionsStyle::Undecided;
use std::mem::discriminant;
// use core::str::find;

type Location = (usize, usize);
type ParsedTable = Vec<Vec<ParsedCell>>;

#[derive(PartialEq, Clone)]
pub enum ParsedCell {
    Label(String),
    Interactive( (String, String, String) ),
}

#[derive(PartialEq, Clone, Deserialize)]
pub enum ExerciseMode {
    Show, // ABC
    Censor, // [][][]
    ClickReveal, // [] -> A
    CensorByLetter, // A[][] -> AB[]
    TypeField, // [Az ]
    DropDown(OptionsStyle), // >ABC or >XYZ
}

impl FromStr for ExerciseMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Show"           => Ok(ExerciseMode::Show),
            "Censor"         => Ok(ExerciseMode::Censor),
            "ClickReveal"    => Ok(ExerciseMode::ClickReveal),
            "CensorByLetter" => Ok(ExerciseMode::CensorByLetter),
            "TypeField"      => Ok(ExerciseMode::TypeField),
            "DropDown"       => Ok(ExerciseMode::DropDown(OptionsStyle::Undecided)),
            _ => Err(())
        }
    }
}

impl ToString for ExerciseMode {
    fn to_string(&self) -> String {
        match &self {
            ExerciseMode::Show => "Show",
            ExerciseMode::Censor => "Censor",
            ExerciseMode::ClickReveal => "ClickReveal",
            ExerciseMode::CensorByLetter => "CensorByLetter",
            ExerciseMode::TypeField => "TypeField",
            ExerciseMode::DropDown(_) => "DropDown",
            _ => ""
        }.to_string()
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct TableProps {
    pub table: DataTable,
    pub initial_mode: ExerciseMode,
}

#[derive(PartialEq, Clone, Deserialize)]
pub enum OptionsStyle {
    Undecided, // TODO eliminate need for this
    Disabled,
    All(Option<Vec<String>>),
    ByCol(Option<Vec<Vec<String>>>),
}

impl FromStr for OptionsStyle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Disabled" => Ok(OptionsStyle::Disabled),
            "All" =>      Ok(OptionsStyle::All(None)),
            "ByCol" =>    Ok(OptionsStyle::ByCol(None)),
            _ => Err(())
        }
    }
}

pub(crate) struct Table {
    pub table: DataTable,
    pub parsed_table: ParsedTable,
    pub location_table: Vec<Vec<Location>>,
    pub mode: ExerciseMode,
    pub options: OptionsStyle,
}

pub enum TableMsg {
    SwitchMode(ExerciseMode),
    Clicked(Location),
}

impl Component for Table {
    type Message = TableMsg;
    type Properties = TableProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            table: ctx.props().table.clone(),
            parsed_table: create_parsed_table(&ctx.props().table),
            location_table: create_location_table(&ctx.props().table),
            mode: ctx.props().initial_mode.clone(),
            options: OptionsStyle::Disabled,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TableMsg::SwitchMode(next_mode) => {
                if self.mode == next_mode {
                    return false;
                }
                self.mode = next_mode
            },
            TableMsg::Clicked(_) => { return false; }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let theme: ThemeContext = ctx.;

        // let mode_state = use_state(|| ctx.props().initial_mode);

        let mode_switcher = |e| //ctx.link().call(move |_|
            {
        //     // let mode_str: Box<String> = Box::from(e.target_dyn_into::<HtmlInputElement>().expect("Unknown Event").value());
        //     // mode_state.set(ExerciseMode::from_str(mode_str.deref().as_str()).expect("Unknown selection"));
        //     log(event.value().as_str());
        //     TableMsg::SwitchMode(ExerciseMode::Censor)
        }
       // )
        ;

        // let on_change1 = {
        //
        //     let m = mode_state.clone();
        //     Callback::from(move |_| mode_state.set(*counter + 1))
        // }

        // let paste_detection = ctx.link().batch_callback(|event| {
        //     if event.key() = "C+V" {
        //
        //     }
        // })

        let (drop_down_option_html, extra_option_html) =
            if self.mode == ExerciseMode::DropDown(OptionsStyle::Disabled) {
                (empty_html(),
                 empty_html())
            } else {
                (html! { <option value="DropDown"> {"Drop down"} </option>},
                 html! { <button class={"side-options"}>{"check"}</button>})
            };

        return html! {
            <div class={"table-area"}>
                <div class={"filler-left"}>
                    // important lesson marker
                </div>
                <div class={"filler-center"}>{ self.html() } </div>
                <div class={"filler-right"}>
                    <select class={"side-options"} value={self.mode.to_string()} onchange={mode_switcher.clone()}>
                        <option value="Show"> {"Reveal"} </option>
                        <option value="Censor"> {"Cover"} </option>
                        <option value="ClickReveal"> {"Click reveal"} </option>
                        <option value="CensorByLetter"> {"Reveal by letter"} </option>
                        <option value="TypeField"> {"Enter text"} </option>
                        { drop_down_option_html }
                    </select>
                    { extra_option_html }
                </div>
            </div>
        }
    }

}

impl Table {

    fn html(&self) -> Html {
        // let row_indices = (0..self.table.len());
        return html! { // 'return' is required for some weird macro reason
            <table class={"exercise-table"}> {
                for self.location_table.iter().map(|row_locations| { html! {
                    <tr> {
                        for row_locations.iter().map(|location|
                            self.mediated_cell(location)
                        )
                    } </tr>
                } })
            } </table>
        }
    }

    fn mediated_cell(&self, location: &Location) -> Html {
        let cell: ParsedCell = (*self.parsed_table.get(location.0).unwrap().get(location.1).unwrap()).clone();

        match cell {
            ParsedCell::Label(val) => html! { <td> {val} </td> },
            ParsedCell::Interactive(split) => {
                let (start, middle, end) = split;

                return match self.mode.clone() {
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
                    ExerciseMode::ClickReveal => {
                        empty_html()
                    }
                    ExerciseMode::CensorByLetter => {
                        empty_html()
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
                    ExerciseMode::DropDown(options_style) => {
                        let onchange = Callback::from(move |e| {
                            // log(&*format!("{}-{}-{}", start, middle, end));
                        });
                        let options = match options_style {
                            OptionsStyle::Undecided => vec![],
                            OptionsStyle::Disabled => vec![],
                            OptionsStyle::All(options) => options.unwrap(),
                            OptionsStyle::ByCol(col_options) => (*col_options.unwrap().get(location.1).unwrap()).clone(),
                        };
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
                }
            }
        }
    }

}


fn create_parsed_table(table: &DataTable) -> ParsedTable {
    table.clone().iter()
        .map(|row: &Vec<String>| row.iter()
            .map(|val: &String| split_bars(val.clone()))
            .collect::<Vec<ParsedCell>>())
        .collect()
}

fn create_location_table(table: &DataTable) -> Vec<Vec<Location>> {
    (0..table.len()).map(|row_index|
        (0..table.get(row_index)
            .map(|row: &Vec<String>| row.len())
            .unwrap_or(0 as usize))
        .map(|col_index| (row_index, col_index))
        .collect())
    .collect()
}

fn split_bars(str: String) -> ParsedCell {
    // let find_fn: fn(&str, &str) -> Option<usize> = find; // compiler bug
    let left: Option<usize> = str.as_str().find("|");
    if left.is_some() {
        let (start, middle) = str.split_at(left.unwrap());
        let (_, middle) = middle.split_at(1);
        let right: Option<usize> = middle.clone().find("|");
        if right.is_some() {
            let (middle, end) = middle.split_at(right.unwrap());
            let (_, end) = end.split_at(1);
            return ParsedCell::Interactive( (start.to_string(), middle.to_string(), end.to_string()) );
        }
    }
    return ParsedCell::Label(str);
}

fn create_options_style(parsed_table: &ParsedTable, options_style: OptionsStyle) -> OptionsStyle {
    match options_style {
        OptionsStyle::Disabled => OptionsStyle::Disabled,
        OptionsStyle::All(_) => OptionsStyle::All(Some(create_options(parsed_table.iter().flat_map(|row: &Vec<ParsedCell>| row).collect()))),
        OptionsStyle::ByCol(_) => {
            let cols = parsed_table.get(0).map(|row: &Vec<ParsedCell>| row.len()).unwrap_or(0 as usize);
            let rotated_table = (0..cols)
                .map(|i| parsed_table.iter()
                    .map(|row: &Vec<ParsedCell>| row.get(i).unwrap())
                    .map(|cell: &ParsedCell| cell)
                    .collect());

            OptionsStyle::ByCol(Some(rotated_table.map(|col: Vec<&ParsedCell>| create_options(col)).collect()))
        }
        _ => OptionsStyle::Undecided
    }
}

fn create_options(unfiltered_options: Vec<&ParsedCell>) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut options: Vec<String> = unfiltered_options.iter()
        .filter(|c: &&&ParsedCell| match c { ParsedCell::Interactive(_) => true, _ => false })
        .map(|c: &&ParsedCell| match c { ParsedCell::Interactive(split) => split.1.clone(), _ => unreachable!() } )
        .unique()
        .collect();
    options.shuffle(&mut rng);
    options
}
