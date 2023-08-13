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
use crate::contexts::{DEFAULT_SELECTION_STRING, DropDownCell, Exercise, ExerciseComponent, ExerciseComponentProps,
                      Exercises, Lesson, Lessons, SpoilerCell, TypeFieldCell, ThemeContext, ThemeKind, ThemeProvider, Toolbar};
use crate::contexts::toolbar::TOOLBAR_HEIGHT;
use crate::contexts::use_theme;
use crate::html_if_some;
use crate::contexts::theme::Theme;

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
// wrap around Component
#[function_component(TableHOC)]
pub fn table_hoc(hoc_props: &TableLayout) -> Html {
    let theme = use_theme().kind();
    let props: ThemedTableProps = ThemedTableProps { theme, table_layout: hoc_props.clone() };
    html! { <Table ..props /> }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ThemedTableProps {
    pub theme: ThemeKind,
    pub table_layout: TableLayout,
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
pub enum OptionsStyle {
    Disabled,
    All { options: Vec<String> },
    ByCol { col_options: Vec<Vec<String>> },
}

pub type InputTable = Vec<Vec<String>>;

pub struct InputTracking {
    pub input_table: InputTable,
    pub check_table: bool,
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

pub(crate) struct Table {
    pub parsed_table: ParsedTable,
    pub location_table: Vec<Vec<Location>>,
    pub input_tracking: Option<InputTracking>,
    pub mode: ExerciseMode,
    pub options_style: OptionsStyle,
}

#[derive(Clone, Debug)]
pub enum TableMsg {
    SwitchMode(ExerciseMode),
    CheckClicked,
    CellClicked(Location),
    InputUpdate(Location, String),
    Error,
}

impl Component for Table {
    type Message = TableMsg;
    type Properties = ThemedTableProps;

    fn create(ctx: &Context<Self>) -> Self {
        // let mut initial_mode = ctx.props().default_mode.clone().map(|s| ExerciseMode::from_str(s.as_str()).unwrap_or(ExerciseMode::Censor)).unwrap_or(ExerciseMode::Censor);

        let parsed_table = create_parsed_table(&ctx.props().table_layout.table);
        let location_table = create_location_table(&ctx.props().table_layout.table);
        let options_summary = create_options_style(ctx.props().table_layout.options_style_type.clone(), &parsed_table, &location_table);
        let uninteractive = !parsed_table.iter().flat_map(|v| v).find(|c| c.is_interactive()).is_some();

        Self {
            // table: ctx.props().table.clone(),
            parsed_table,
            location_table,
            input_tracking: options_summary.1.map(|input_table| InputTracking { input_table, check_table: false } ),
            mode: ctx.props().table_layout.default_mode.clone().unwrap_or(if uninteractive {ExerciseMode::Disabled} else {ExerciseMode::ClickReveal}),
            options_style: options_summary.0,
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
                    Some(_) => {
                        self.input_tracking.as_mut().unwrap().check_table =
                            !self.input_tracking.as_mut().unwrap().check_table;
                    }
                };
                true
            },
            TableMsg::InputUpdate(location, value) => {
                match &self.input_tracking {
                    None => unreachable!("updated cell without input tracking"),
                    Some(_) => {
                        self.input_tracking.as_mut().unwrap().input_table.set_location_unchecked(&location, &value);
                        true
                    }
                }
            },
            TableMsg::CellClicked(_) => { false },
            TableMsg::Error => { false },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let theme = &ctx.props().theme;
        let table_area = theme.css_class_themed("table-area");
        let table_secondary_classes = theme.css_class_themed("table-secondary");
        let mut side_options_class = theme.css_class_themed("side-options");
        // let mut check = side_options_class.clone();
        // check.push_str(" check");

        let check_clicked_class = self.is_checking().then_some(theme.css_class_themed("check_clicked_class"));

        let mode_switcher = ctx.link().callback(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            ExerciseMode::from_str(input.value().as_str()).ok()
                .map(TableMsg::SwitchMode)
                .unwrap_or(TableMsg::Error)
        });

        let check_answers = ctx.link().callback(move |_e: MouseEvent| {
            TableMsg::CheckClicked
        });

        // let paste_detection = ctx.link().batch_callback(|event| {
        //     if event.key() = "V" && event.ctrl() {
        //
        //     }
        // })
        let disabled = self.mode == ExerciseMode::Disabled;

        return html! {
            <div class={table_area}>
                <div class={"filler-left"}>
                    // important lesson marker
                </div>
                <div class={"filler-center"}>{ self.table_html(ctx) } </div>
                <div class={"filler-right table-right"}>
                    if !disabled {
                        if self.mode.has_input() {
                            <button class={classes!(side_options_class.clone(), "check", check_clicked_class)} onclick={check_answers}> {"check"} </button>
                        }
                        <select class={side_options_class} value={self.mode.to_string().clone()} onchange={mode_switcher.clone()}>
                            <option value="Show"           selected={"Show" == self.mode.to_string().clone()}>            {"Reveal all"} </option>
                            <option value="HoverReveal"    selected={"HoverReveal" == self.mode.to_string().clone()}>     {"Hover reveal"} </option>
                            <option value="ClickReveal"    selected={"ClickReveal" == self.mode.to_string().clone()}>     {"Click reveal"} </option>
                         // <option value="CensorByLetter" selected={"CensorByLetter" == self.mode.to_string().clone()}>  {"Reveal by letter"} </option>
                            <option value="TypeField"      selected={"TypeField" == self.mode.to_string().clone()}>       {"Enter text"} </option>
                            <option value="DropDown"       selected={"DropDown" == self.mode.to_string().clone()} disabled={self.options_style == OptionsStyle::Disabled}> {"Drop down"} </option>
                        </select>
                    }
                </div>
            </div>
        }
    }

}

impl Table {

    fn is_checking(&self) -> bool {
        match &self.input_tracking {
            None => false,
            Some(i) => i.check_table
        }
    }

    fn is_checking_unwrap(&self) -> bool {
        self.input_tracking.as_ref().unwrap().check_table
    }

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

    fn mediated_cell(&self, location: &Location, _ctx: &Context<Self>) -> Html {
        let cell: ParsedCell = (*self.parsed_table.get(location.0).unwrap().get(location.1).unwrap()).clone();
        let theme = &_ctx.props().theme;
        // let table_secondary_classes = theme.css_class_themed("table-secondary");
        let table_input = theme.css_class_themed("table-input");

        match cell {
            ParsedCell::Label(val) => html! { <td> { val } </td> },
            ParsedCell::Interactive(text) => {
                return match self.mode.clone() {
                    ExerciseMode::Show => html! { <td class={theme.css_class_themed("interactive")}> { text.start }  { text.middle } { text.end } </td> },
                    ExerciseMode::HoverReveal => html! { <td class={theme.css_class_themed("spoilable")}> { text.start } <span class={theme.css_class_themed("spoiler")}> { text.middle } </span> { text.end } </td> },
                    ExerciseMode::ClickReveal => html! { <SpoilerCell text={text} class={theme.css_class_themed("spoilable")} /> },
                    ExerciseMode::CensorByLetter => { empty_html() }
                    ExerciseMode::TypeField | ExerciseMode::DropDown => {
                        if let Some(key_col) = _ctx.props().table_layout.key_col {
                            if location.1 == key_col {
                                return html! { <td> { text.start }  { text.middle } { text.end } </td> }
                            }
                        }

                        let check_mode = self.is_checking_unwrap();

                        match self.mode.clone() {
                            ExerciseMode::TypeField => html! { <TypeFieldCell text={text} class={table_input} check_mode={check_mode} /> },
                            ExerciseMode::DropDown => {
                                let options = match self.options_style.clone() {
                                    OptionsStyle::Disabled => unreachable!("Accessed drop down when it was disabled"),
                                    OptionsStyle::All { options } => options.clone(),
                                    OptionsStyle::ByCol { col_options } => col_options.get(location.1).unwrap().clone(),
                                };

                                html! { <DropDownCell text={text.clone()} class={table_input} location={location.clone()} options={options} check_mode={check_mode} /> }
                            }
                            _ => unreachable!()
                        }
                    },
                    ExerciseMode::Disabled => unreachable!("creating disabled exercise mode"),
                }
            }
        }
    }


}

fn create_options_style(options_style_type: Option<OptionsStyleType>, parsed_table: &ParsedTable, location_table: &Vec<Vec<Location>>) -> (OptionsStyle, Option<InputTable>) {
    let options_style_type_predicted: OptionsStyleType = options_style_type.unwrap_or({ // predict the default based on the data
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
                    OptionsStyleType::ByCol
                } else {
                    OptionsStyleType::All
                }
            } else {
                OptionsStyleType::All
            }
        } else {
            OptionsStyleType::Disabled // no cells are interactive
        }
    });

    match options_style_type_predicted {
        OptionsStyleType::Disabled => (OptionsStyle::Disabled, None),
        OptionsStyleType::All => {
            let options = create_options(parsed_table.iter().flat_map(|row: &Vec<ParsedCell>| row).collect());

            // let selected_option = options.first().map(|s: &String| s.clone().to_owned()).unwrap_or(String::new());
            // let input_table = (0..parsed_table.len()).map(|r: usize| (0..parsed_table.get(r).unwrap().len()).map(|_| selected_option.clone()).collect()).collect();
            let initial_input_table = (0..parsed_table.len()).map(|r: usize| (0..parsed_table.get(r).unwrap().len()).map(|_| DEFAULT_SELECTION_STRING.clone()).collect()).collect();

            (if options.len() > 1 { OptionsStyle::All { options } } else { OptionsStyle::Disabled },
                Some(initial_input_table)
            )
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

            // let selected_row_option: Vec<String> = col_options.iter().map(|v: &Vec<String>| v.first().map(|s: &String| s.clone().to_owned()).unwrap()).collect();
            // let input_table = (0..parsed_table.len()).map(|r: usize| (0..parsed_table.get(r).unwrap().len()).map(|c: usize| selected_row_option.get(c).unwrap().clone()).collect()).collect();

            let initial_input_table = (0..parsed_table.len()).map(|r: usize| (0..parsed_table.get(r).unwrap().len()).map(|_| DEFAULT_SELECTION_STRING.clone()).collect()).collect();
            (OptionsStyle::ByCol { col_options }, Some(initial_input_table))
        }
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
    fn has_input(&self) -> bool {
        match self {
            ExerciseMode::TypeField => true,
            ExerciseMode::DropDown => true,
            _ => false,
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
            "Disabled"       => Ok(ExerciseMode::Disabled),
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
        ExerciseMode::Disabled =>       "Disabled",
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
