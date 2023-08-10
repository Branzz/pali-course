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
                      Lessons, Lesson, Exercise, Exercises, SpoilerCell};
use crate::{get_lessons_json, log_str, log_display, log_dbg};
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
use std::mem::discriminant;

// use core::str::find;

type DataTable = Vec<Vec<String>>;

#[derive(Properties, PartialEq, Clone, Deserialize)]
pub struct TableProps {
    // pub table: DataTable,
    // pub initial_mode: ExerciseMode,
    pub table: DataTable,
    pub default_mode: Option<ExerciseMode>, // Default: Censor
    pub options_style: Option<OptionsStyle>, // Default: All
}

#[derive(PartialEq, Clone, Deserialize)]
#[serde(tag = "type", content = "options")]
pub enum OptionsStyle {
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
            _ =>          Ok(OptionsStyle::Disabled),
        }
    }
}

type ParsedTable = Vec<Vec<ParsedCell>>;
type Location = (usize, usize);

trait DirectionalInclusive<T> {
    fn left (&self, of: T) -> bool;
    fn right(&self, of: T) -> bool;
    fn above(&self, of: T) -> bool;
    fn below(&self, of: T) -> bool;
}

impl DirectionalInclusive<&Location> for Location {
    fn left (&self, of: &Location) -> bool {
        self.1 < of.1
    }
    fn right(&self, of: &Location) -> bool {
        self.1 > of.1
    }
    fn above(&self, of: &Location) -> bool {
        self.0 < of.0
    }
    fn below(&self, of: &Location) -> bool {
        self.0 > of.0
    }
}


trait GetLocation<T> {
    fn get_location(&self, location: &Location) -> Option<&T>;
    fn get_location_unchecked(&self, location: &Location) -> &T;
}

impl<T> GetLocation<T> for Vec<Vec<T>> {
    fn get_location(&self, location: &Location) -> Option<&T> {
        self.get(location.0).map(|v| v.get(location.1)).flatten()
    }

    fn get_location_unchecked(&self, location: &Location) -> &T {
        self.get(location.0).unwrap().get(location.1).unwrap()
    }
}

pub(crate) struct Table {
    // pub table: DataTable,
    pub parsed_table: ParsedTable,
    pub location_table: Vec<Vec<Location>>,
    pub mode: ExerciseMode,
    pub options_style: OptionsStyle,
}

#[derive(Clone, Debug)]
pub enum TableMsg {
    SwitchMode(ExerciseMode),
    CheckClicked,
    CellClicked(Location),
    Error,
}

impl Component for Table {
    type Message = TableMsg;
    type Properties = TableProps;

    fn create(ctx: &Context<Self>) -> Self {
        // let mut initial_mode = ctx.props().default_mode.clone().map(|s| ExerciseMode::from_str(s.as_str()).unwrap_or(ExerciseMode::Censor)).unwrap_or(ExerciseMode::Censor);

        let parsed_table = create_parsed_table(&ctx.props().table);
        let location_table = create_location_table(&ctx.props().table);
        let options_style = create_options_style(ctx.props().options_style.clone(), &parsed_table, &location_table);
        Self {
            // table: ctx.props().table.clone(),
            parsed_table,
            location_table,
            mode: ctx.props().default_mode.clone().unwrap_or(ExerciseMode::ClickReveal),
            options_style,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TableMsg::SwitchMode(next_mode) => {
                if self.mode == next_mode {
                    return false;
                }
                self.mode = next_mode;

                true
            },
            TableMsg::CheckClicked => { true },
            TableMsg::CellClicked(_) => { false },
            TableMsg::Error => { false },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let theme: ThemeContext = ctx.;

        // let mode_state = use_state(|| ctx.props().initial_mode);

        let mode_switcher = ctx.link().callback(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            ExerciseMode::from_str(input.value().as_str()).ok()
                .map(TableMsg::SwitchMode)
                .unwrap_or(TableMsg::Error)
        });

        let check_answers = ctx.link().callback(move |e: MouseEvent| {
            TableMsg::CheckClicked
        });

        // let paste_detection = ctx.link().batch_callback(|event| {
        //     if event.key() = "C+V" {
        //
        //     }
        // })

        return html! {
            <div class={"table-area"}>
                <div class={"filler-left"}>
                    // important lesson marker
                </div>
                <div class={"filler-center"}>{ self.table_html(ctx) } </div>
                <div class={"filler-right"}>
                    <select class={"side-options"} value={self.mode.to_string()} onchange={mode_switcher.clone()}>
                        <option value="Show"> {"Reveal all"} </option>
                        <option value="HoverReveal"> {"Hover reveal"} </option>
                        <option value="ClickReveal"> {"Click reveal"} </option>
                        <option value="CensorByLetter"> {"Reveal by letter"} </option>
                        <option value="TypeField"> {"Enter text"} </option>
                        if self.options_style != OptionsStyle::Disabled {
                           <option value="DropDown"> {"Drop down"} </option>
                        }
                    </select>
                    if self.options_style != OptionsStyle::Disabled
                            && self.mode.has_input() {
                        <button class={"side-options"} onclick={check_answers}> {"check"} </button> // TODO interaction
                    }
                </div>
            </div>
        }
    }

}

impl Table {

    fn table_html(&self, ctx: &Context<Self>) -> Html {
        // let row_indices = (0..self.table.len());
        return html! { // 'return' is required for some weird macro reason
            <table class={"exercise-table"}> {
                for self.location_table.iter().map(|row_locations| { html! {
                    <tr> {
                        for row_locations.iter().map(|location|
                            self.mediated_cell(location, ctx)
                        )
                    } </tr>
                } })
            } </table>
        }
    }

    fn mediated_cell(&self, location: &Location, ctx: &Context<Self>) -> Html {
        let cell: ParsedCell = (*self.parsed_table.get(location.0).unwrap().get(location.1).unwrap()).clone();

        match cell {
            ParsedCell::Label(val) => html! { <td> {val} </td> },
            ParsedCell::Interactive(text) => {
                // let (start, middle, end) = split;

                return match self.mode.clone() {
                    ExerciseMode::Show => {
                        (html! {
                            <td class={"interactive"}> { text.start }  { text.middle } { text.end } </td>
                        }) as Html
                    }
                    ExerciseMode::HoverReveal => {
                        (html! {
                            <td class={"spoilable"}> { text.start } <span class={"spoiler"}> { text.middle } </span> { text.end } </td>
                        }) as Html
                    }
                    ExerciseMode::ClickReveal => {
                        (html! { <SpoilerCell text={text}/> }) as Html
                    }
                    ExerciseMode::CensorByLetter => {
                        empty_html()
                    }
                    ExerciseMode::TypeField => {
                        // https://jsfiddle.net/drq0nz6j/
                        (html! {
                            <td> { text.start } <input class={"table-input"} type="text"/> { text.end } </td>
                        }) as Html
                    }
                    ExerciseMode::DropDown => {
                        let options = match self.options_style.clone() {
                            OptionsStyle::Disabled => unreachable!("Accessed Dropdown when it was disabled"),
                            OptionsStyle::All(options)=> options.unwrap(),
                            OptionsStyle::ByCol(col_options)=> (*col_options.unwrap().get(location.1).unwrap()).clone(),
                        };
                        (html! {
                            <td> { text.start }
                                <select class={"table-input"}>
                                    { for options.iter().map(|o| { html! {
                                        <option value={o.clone()}>{o}</option>
                                    } }) }
                                </select>
                            { text.end } </td>
                        }) as Html
                    }
                }
            }
        }
    }

}

fn create_options_style(from_options_style_unpredicted: Option<OptionsStyle>, parsed_table: &ParsedTable, location_table: &Vec<Vec<Location>>) -> OptionsStyle {
    let from_options_style: OptionsStyle = from_options_style_unpredicted.unwrap_or({ // predict the default based on the data
        let top_left_opt: Option<&Location> = location_table.iter().flat_map(|v| v)
            .find(|l: &&Location| parsed_table.get_location_unchecked(l).is_interactive());
        if let Some(top_left) = top_left_opt {
            let bottom_right: &Location = location_table.iter().flat_map(|v| v)
                .rev()
                .find(|l: &&Location| parsed_table.get_location_unchecked(l).is_interactive()).unwrap();

            let extends_horizontally = top_left.1 == 0 && bottom_right.1 == parsed_table.get(bottom_right.0).unwrap().len() - 1;

            if extends_horizontally {
                let forms_a_grid = log_dbg(location_table.iter().flat_map(|v| v).find(|loc: &&Location| {
                    // let loc: Location = *loc_ref.clone();
                    if parsed_table.get_location_unchecked(loc).is_interactive()
                        { loc.left(top_left) || loc.above(top_left) || loc.right(bottom_right) || loc.below(bottom_right) }
                    else
                        { !loc.left(top_left) && !loc.above(top_left) && !loc.right(bottom_right) && !loc.below(bottom_right) }
                })).is_none();

                if forms_a_grid {
                    OptionsStyle::ByCol(None)
                } else {
                    OptionsStyle::All(None)
                }
            } else {
                OptionsStyle::All(None)
            }
        } else {
            OptionsStyle::Disabled // no cells are interactive
        }
    } as OptionsStyle);

    match from_options_style {
        OptionsStyle::Disabled => OptionsStyle::Disabled,
        OptionsStyle::All(_) => {
            let options = create_options(parsed_table.iter().flat_map(|row: &Vec<ParsedCell>| row).collect());
            if options.len() > 1 { OptionsStyle::All(Some(options))} else { OptionsStyle::Disabled }
        },
        OptionsStyle::ByCol(_) => {
            let cols = parsed_table.get(0).map(|row: &Vec<ParsedCell>| row.len()).unwrap_or(0 as usize);
            let rotated_table = (0..cols)
                .map(|i| parsed_table.iter()
                    .map(|row: &Vec<ParsedCell>| row.get(i))
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .map(|cell: &ParsedCell| cell)
                    .collect());

            let options = rotated_table.map(|col: Vec<&ParsedCell>| create_options(col)).collect();
            OptionsStyle::ByCol(Some(options))
        }
        _ => OptionsStyle::Disabled
    }
}

#[derive(PartialEq, Clone)]
pub struct TriSplit {
    pub start: String,
    pub middle: String,
    pub end: String,
}

impl TriSplit {
    pub fn new(start: String, middle: String, end: String) -> Self {
        Self { start, middle, end }
    }
}

#[derive(PartialEq, Clone)]
pub enum ParsedCell {
    Label(String),
    Interactive(TriSplit),
}

impl ParsedCell {
    fn is_interactive(&self) -> bool {
        match self {
            ParsedCell::Interactive(_) => true,
            _ => false
        }
    }
}

#[derive(PartialEq, Clone, Deserialize, Debug)]
pub enum ExerciseMode {
    Show, // ABC
    HoverReveal, // [][][]
    ClickReveal, // [] -> A
    CensorByLetter, // A[][] -> AB[]
    TypeField, // [Az ]
    DropDown, // >ABC or >XYZ
}

impl ExerciseMode {
    fn has_input(&self) -> bool {
        match self {
            _ => false,
            ExerciseMode::TypeField => true,
            ExerciseMode::DropDown => true,
        }
    }
}

impl FromStr for ExerciseMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Show"           => Ok(ExerciseMode::Show),
            "HoverReveal"    => Ok(ExerciseMode::HoverReveal),
            "ClickReveal"    => Ok(ExerciseMode::ClickReveal),
            "CensorByLetter" => Ok(ExerciseMode::CensorByLetter),
            "TypeField"      => Ok(ExerciseMode::TypeField),
            "DropDown"       => Ok(ExerciseMode::DropDown),
            _ => Err(())
        }
    }
}

impl ToString for ExerciseMode {
    fn to_string(&self) -> String {
        match self {
        ExerciseMode::Show =>           "Show",
        ExerciseMode::HoverReveal =>    "HoverReveal",
        ExerciseMode::ClickReveal =>    "ClickReveal",
        ExerciseMode::CensorByLetter => "CensorByLetter",
        ExerciseMode::TypeField =>      "TypeField",
        ExerciseMode::DropDown =>       "DropDown",
            _ => ""
        }.to_string()
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
            return ParsedCell::Interactive(TriSplit::new(start.to_string(), middle.to_string(), end.to_string()));
        }
    }
    return ParsedCell::Label(str);
}

fn create_options(unfiltered_options: Vec<&ParsedCell>) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut options: Vec<String> = unfiltered_options.iter()
        .filter(|c: &&&ParsedCell| c.is_interactive())
        .map(|c: &&ParsedCell| match c { ParsedCell::Interactive(text) => text.middle.clone(), _ => unreachable!() } )
        .unique()
        .collect();
    options.shuffle(&mut rng);
    options
}
