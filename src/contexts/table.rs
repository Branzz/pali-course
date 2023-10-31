use std::collections::HashSet;
use std::mem::discriminant;
use std::ops::Deref;
use std::panic;
use std::slice::Iter;
use std::str::FromStr;
use std::str::pattern::{Pattern, Searcher, SearchStep};
use gloo_net::http::Request;
use itertools::{Itertools, Unique};
use percent_encoding::percent_decode_str;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use stylist::yew::styled_component;
use wasm_bindgen::JsValue;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{Callback, Html, html, Properties, use_state};
use yew::prelude::*;
use yew::props;
use yew_router::prelude::*;

use crate::{get_lessons_json, log_dbg, log_display, log_str};
use crate::app::empty_html;
use crate::contexts::{DEFAULT_SELECTION_STRING, DropDownCell, Exercise, ExerciseComponent, ExerciseComponentProps, Exercises, Lesson, Lessons, SpoilerCell, TypeFieldCell, ThemeContext, ThemeKind, ThemeProvider, Toolbar, ExerciseCategory};
use crate::contexts::toolbar::TOOLBAR_HEIGHT;
use crate::contexts::use_theme;
use crate::html_if_some;
use crate::contexts::theme::Theme;
use std::borrow::BorrowMut;
use ExerciseMode::*;

type DataTable = Vec<Vec<String>>;

#[derive(Properties, PartialEq, Clone, Deserialize)]
pub struct TableLayout {
    // pub table: DataTable,
    // pub initial_mode: ExerciseMode,
    pub table: DataTable,
    pub key_col: Option<usize>, // Show when rows unordered
    pub shuffle_rows: Option<bool>,
    pub default_mode: Option<ExerciseMode>, // Default: Censor
    pub options_style_type: Option<OptionsStyleType>, // predicted
}

#[derive(Properties, PartialEq, Clone)]
pub struct ThemedTableProps {
    pub theme: ThemeKind,
    pub table_layout: TableLayout,
    pub categories: Vec<ExerciseCategory>,
    pub id: String,
}


pub(crate) struct Table {
    pub parsed_table: ParsedTable,
    pub location_table: Vec<Vec<Location>>,
    pub input_tracking: Option<bool>,
    pub reset: bool,
    pub mode: ExerciseMode,
    pub options_style: DropDownOptionsStyle,
    pub type_field_size: Vec<i32>, // by column
}

#[derive(Clone, Debug)]
pub enum TableMsg {
    SwitchMode(ExerciseMode),
    CheckClicked,
    CellClicked(Location),
    Reset,
    Error,
}

impl Component for Table {
    type Message = TableMsg;
    type Properties = ThemedTableProps;

    fn create(ctx: &Context<Self>) -> Self {
        let parsed_table = create_parsed_table(&ctx.props().table_layout.table);
        let location_table = create_location_table(&ctx.props().table_layout.table);
        let options_summary = create_options_style(ctx.props().table_layout.options_style_type.clone(), &parsed_table, &location_table);
        let interactive = parsed_table.iter().flat_map(|v| v).find(|c| c.is_interactive()).is_some();
        let type_field_size = max_length(&parsed_table);
        let mode = ctx.props().table_layout.default_mode.clone().unwrap_or(
            if interactive {
                if ctx.props().categories.contains(&ExerciseCategory::Conjugation) {
                    ExerciseMode::HoverReveal
                } else { // Vocab, Verbs, other...
                    ClickReveal
                }
            } else {
                Disabled
            }
        );

        Self {
            parsed_table,
            location_table,
            input_tracking: if interactive {Some(false)} else {None},
            reset: false,
            mode,
            options_style: options_summary,
            type_field_size,
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
            TableMsg::CheckClicked => {
                match &self.input_tracking {
                    None => unreachable!("clicked check with no input tracking"),
                    Some(prev) => {
                        self.input_tracking = Some(!prev.clone());
                    }
                };
                true
            },
            TableMsg::Reset => {
                self.reset = !self.reset;
                if self.input_tracking.is_some() {
                    self.input_tracking = Some(false);
                }
                true
            }
            TableMsg::CellClicked(_) => { false },
            TableMsg::Error => { false },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let theme = &ctx.props().theme;
        let table_area = theme.css_class_themed("table-area");
        // let table_secondary_classes = theme.css_class_themed("table-secondary");
        let side_options_class = theme.css_class_themed("side-options");
        // let mut check = side_options_class.clone();
        // check.push_str(" check");

        let check_clicked_class = self.is_checking().then_some(theme.css_class_themed("check_clicked_class"));
        let mode_switcher = ctx.link().callback(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            ExerciseMode::from_str(input.value().as_str()).ok()
                .map(TableMsg::SwitchMode)
                .unwrap_or(TableMsg::Error)
        });
        let check_answers = ctx.link().callback(move |_: MouseEvent| TableMsg::CheckClicked);
        let reset = ctx.link().callback(move |_: MouseEvent| TableMsg::Reset);
        let disabled = self.mode == Disabled;

        let html = html! {
            <div class={table_area}>
                <div class="filler-left">
                    // important lesson marker
                </div>
                <div class="filler-center filler-table">{ self.table_html(ctx) } </div>
                <div class="filler-right table-right">
                    if !disabled {
                        if self.mode.has_input() {
                            <button class={classes!("check", side_options_class.clone(), "side-button", check_clicked_class.clone())} onclick={check_answers}> {"check"} </button>
                        }
                        <select class={classes!("options", side_options_class.clone(), "clickable")} value={self.mode.to_string().clone()} onchange={mode_switcher.clone()}>
                            <option value="Show"           selected={"Show" == self.mode.to_string().clone()}>            {"Reveal all"} </option>
                            <option value="HoverReveal"    selected={"HoverReveal" == self.mode.to_string().clone()}>     {"Hover reveal"} </option>
                            <option value="ClickReveal"    selected={"ClickReveal" == self.mode.to_string().clone()}>     {"Click reveal"} </option>
                         // <option value="CensorByLetter" selected={"CensorByLetter" == self.mode.to_string().clone()}>  {"Reveal by letter"} </option>
                            <option value="TypeField"      selected={"TypeField" == self.mode.to_string().clone()}>       {"Enter text"} </option>
                            <option value="DropDown"       selected={"DropDown" == self.mode.to_string().clone()} disabled={self.options_style == DropDownOptionsStyle::Disabled}> {"Drop down"} </option>
                        </select>
                        if self.mode.is_resettable() {
                            <button class={classes!("reset", side_options_class, "side-button")} onclick={reset}> {"↺"} </button>
                        }
                    }
                </div>
            </div>
        };

        // ctx.link().callback(move |_| TableMsg::Reset).emit(());

        html
    }

}

impl Table {

    fn is_checking(&self) -> bool {
        match &self.input_tracking {
            None => false,
            Some(checking) => *checking
        }
    }

    fn is_checking_unwrap(&self) -> bool {
        *self.input_tracking.as_ref().unwrap()
    }

    fn table_html(&self, ctx: &Context<Self>) -> Html {
        // let row_indices = (0..self.table.len());
        return html! { // 'return' is required for some weird macro reason
            <table class="exercise-table"> {
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
        let theme = &ctx.props().theme;
        // let table_secondary_classes = theme.css_class_themed("table-secondary");
        let mut table_input = theme.css_class_themed("table-input");

        match cell {
            ParsedCell::Label(val) => html! { <td> { val } </td> },
            ParsedCell::Interactive(text) => {
                return match self.mode.clone() {
                    Show => html! { <td class={theme.css_class_themed("interactive")}> { text.start }  { text.middle } { text.end } </td> },
                    HoverReveal => html! { <td class={theme.css_class_themed("spoilable")}> { text.start } <span class={theme.css_class_themed("spoiler")}> { text.middle } </span> { text.end } </td> },
                    CensorByLetter => { empty_html() }

                    ClickReveal | TypeField | DropDown => {

                        // you could technically "hack" this with a clever enough key from json titles, but it'll just make two elements show the same thing
                        let key = format!("{}-{}-{},{}{}", ctx.props().id.clone(), self.mode.to_string(), location.0, location.1, self.reset);

                        match self.mode.clone() {
                            ClickReveal => html! { <SpoilerCell text={text} theme={theme} class={theme.css_class_themed("spoilable")} key={key} /> },
                            TypeField | DropDown => {
                                if let Some(key_col) = ctx.props().table_layout.key_col {
                                    if location.1 == key_col {
                                        return html! { <td> { text.start }  { text.middle } { text.end } </td> }
                                    }
                                }

                                let check_mode = self.is_checking_unwrap();

                                match self.mode.clone() {
                                    TypeField => {
                                        table_input.push_str(" type-field");
                                        html! { <TypeFieldCell text={text} class={table_input} check_mode={check_mode} size={self.type_field_size[location.1]} key={key} /> }
                                    },
                                    DropDown => {
                                        let options = match self.options_style.clone() {
                                            DropDownOptionsStyle::Disabled => unreachable!("Accessed drop down when it was disabled"),
                                            DropDownOptionsStyle::All { options } => options.clone(),
                                            DropDownOptionsStyle::ByCol { col_options } => col_options.get(location.1).unwrap().clone(),
                                        };

                                        html! { <DropDownCell text={text.clone()} class={table_input} location={location.clone()} options={options} check_mode={check_mode} key={key} /> }
                                    }
                                    _ => unreachable!()
                                }
                            }
                            _ => unreachable!()
                        }
                    },
                    Disabled => unreachable!("creating disabled exercise mode"),
                }
            }
        }
    }


}

fn create_options_style(options_style_type: Option<OptionsStyleType>, parsed_table: &ParsedTable, location_table: &Vec<Vec<Location>>) -> DropDownOptionsStyle {
    match predict_options_style_type(options_style_type, parsed_table, location_table) {
        OptionsStyleType::Disabled => DropDownOptionsStyle::Disabled,
        OptionsStyleType::All => {
            let options = create_options(parsed_table.iter().flat_map(|row: &Vec<ParsedCell>| row).collect());
            if options.len() > 1 { DropDownOptionsStyle::All { options } } else { DropDownOptionsStyle::Disabled }
        },
        OptionsStyleType::ByCol => {
            let max_columns = parsed_table.iter().map(|row| row.len()).max().unwrap_or(0);
            let mut col_options: Vec<Vec<&ParsedCell>> = vec![Vec::new(); max_columns];
            for row in parsed_table {
                for (col_index, cell) in row.iter().enumerate() {
                    col_options[col_index].push(cell);
                }
            }
            let col_options: Vec<Vec<String>> = col_options.iter().map(|col| create_options(col.iter().copied().collect())).collect();
            DropDownOptionsStyle::ByCol { col_options }
        }
    }
}

fn predict_options_style_type(options_style_type: Option<OptionsStyleType>, parsed_table: &ParsedTable, location_table: &Vec<Vec<Location>>) -> OptionsStyleType {
    if options_style_type.is_some() {
        return options_style_type.unwrap();
    }

    let top_left_opt: Option<&Location> = location_table.iter().flat_map(|v| v)
        .find(|l: &&Location| parsed_table.get_location_unchecked(l).is_interactive());
    if top_left_opt.is_none() {
        return OptionsStyleType::Disabled; // no cells are interactive
    }
    let top_left = top_left_opt.unwrap();

    let bottom_right: &Location = location_table.iter().flat_map(|v| v)
        .rev()
        .find(|l: &&Location| parsed_table.get_location_unchecked(l).is_interactive()).unwrap();

    let extends_horizontally = top_left.1 == 0 && bottom_right.1 == parsed_table.get(bottom_right.0).unwrap().len() - 1;

    if extends_horizontally {
        let forms_a_grid = location_table.iter().flat_map(|v| v).find(|loc: &&Location| {
            // let loc: Location = *loc_ref.clone();
            if parsed_table.get_location_unchecked(loc).is_interactive()
            { loc.left(top_left) || loc.above(top_left) || loc.right(bottom_right) || loc.below(bottom_right) }
            else
            { !loc.left(top_left) && !loc.above(top_left) && !loc.right(bottom_right) && !loc.below(bottom_right) }
        }).is_none();

        if forms_a_grid {
            return OptionsStyleType::ByCol;
        }
    }
    return OptionsStyleType::All;
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
        let right: Option<usize> = middle.find("|");
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

fn max_length(table: &ParsedTable) -> Vec<i32> {
    // (*ctx.props().table_layout.table.iter()
    //     .map(|row: Vec<String>| )
    //     .map().map(|u| u as i32).fold(0, |a, b| a.max(*b))),
    let max_columns = table.iter().map(|row| row.len()).max().unwrap_or(0);

    let mut column_max_sizes: Vec<i32> = vec![0; max_columns];

    for row in table {
        for (col_index, cell) in row.iter().enumerate() {
            if let ParsedCell::Interactive(split) = cell {
                let len = split.middle.len() as i32;
                if len > column_max_sizes[col_index] {
                    column_max_sizes[col_index] = len;
                }
            }
        }
    }

    for i in 0..column_max_sizes.len() {
        column_max_sizes[i] = column_max_sizes[i] / 2;
    }

    return column_max_sizes;
}

#[derive(PartialEq, Clone, Deserialize)]
#[serde(tag = "type", content = "options")]
/// for DropDown mode
pub enum OptionsStyleType {
    Disabled,
    All,
    ByCol,
}

#[derive(PartialEq, Clone, Deserialize)]
#[serde(tag = "type", content = "options")]
/// for DropDown mode
pub enum DropDownOptionsStyle {
    Disabled,
    All { options: Vec<String> },
    ByCol { col_options: Vec<Vec<String>> },
}

impl FromStr for OptionsStyleType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Disabled" => Ok(OptionsStyleType::Disabled),
            "All" =>      Ok(OptionsStyleType::All),
            "ByCol" =>    Ok(OptionsStyleType::ByCol),
            _ =>          Err(()),
        }
    }
}

type ParsedTable = Vec<Vec<ParsedCell>>;
pub(crate) type Location = (usize, usize);

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

trait SetLocation<T> {
    fn set_location_unchecked(&mut self, location: &Location, value: &T);
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

impl<T> SetLocation<T> for Vec<Vec<T>> {
    fn set_location_unchecked(&mut self, location: &Location, value: &T) {
        let row: &Vec<T> = self.get(location.0).as_mut().unwrap();
        let _ = std::mem::replace(&mut &row[location.1], value);
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
    Disabled,
    Show, // ABC
    HoverReveal, // [][][]
    ClickReveal, // [] -> A
    CensorByLetter, // A[][] -> AB[]
    TypeField, // [Az ]
    DropDown, // >ABC or >XYZ
}

impl ExerciseMode {

    fn is_resettable(&self) -> bool {
        match self {
            ClickReveal
            | TypeField
            | DropDown => true,
            _ => false,
        }
    }

    fn has_input(&self) -> bool {
        match self {
            TypeField
            | DropDown => true,
            _ => false,
        }
    }

}

impl FromStr for ExerciseMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Show"           => Ok(Show),
            "HoverReveal"    => Ok(HoverReveal),
            "ClickReveal"    => Ok(ClickReveal),
            "CensorByLetter" => Ok(CensorByLetter),
            "TypeField"      => Ok(TypeField),
            "DropDown"       => Ok(DropDown),
            "Disabled"       => Ok(Disabled),
            _ => Err(())
        }
    }
}

impl ToString for ExerciseMode {
    fn to_string(&self) -> String {
        match self {
            Show =>           "Show",
            HoverReveal =>    "HoverReveal",
            ClickReveal =>    "ClickReveal",
            CensorByLetter => "CensorByLetter",
            TypeField =>      "TypeField",
            DropDown =>       "DropDown",
            Disabled =>       "Disabled",
        }.to_string()
    }
}
