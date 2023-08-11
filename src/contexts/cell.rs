use yew::{Context, Component, Html, html, Properties};
use crate::contexts::table::{Location, InputTracking};
use crate::contexts::TriSplit;
use crate::{log, log_display};
use yew::prelude::*;
use web_sys::{MouseEvent, HtmlInputElement};

#[derive(Properties, PartialEq)]
pub struct SpoilerCellProps {
    pub text: TriSplit,
}

pub struct SpoilerCell {
    spoiled: bool,
}

impl SpoilerCell {
}

pub enum SpoilerCellMsg {
    FlipState,
    Spoil,
    Reveal,
}

impl Component for SpoilerCell {
    type Message = SpoilerCellMsg;
    type Properties = SpoilerCellProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { spoiled: true }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SpoilerCellMsg::FlipState => { self.spoiled = !self.spoiled; true },
            SpoilerCellMsg::Spoil => { if self.spoiled { false } else { self.spoiled = false; true }},
            SpoilerCellMsg::Reveal => { if !self.spoiled { false } else { self.spoiled = true; true }},
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(move |e: MouseEvent| {
            SpoilerCellMsg::FlipState
        });

        let spoil_class = if self.spoiled { "spoiler_button invisible" } else { "spoiler_button visible" };
        let text = ctx.props().text.clone();

        html! {
            <td class={"interactive"} onmousedown={onclick.clone()}> { text.start } <span class={spoil_class} onmousedown={onclick}> { text.middle } </span> { text.end } </td>
        }
    }

}

pub(crate) const DEFAULT_SELECTION_STRING: String = String::new();

#[derive(Properties, PartialEq)]
pub struct InteractiveCellProps {
    pub text: TriSplit,
    pub options: Vec<String>,
    pub location: Location,
    pub check_mode: bool,
}

pub struct DropDownCell {
    pub selected: String,
}

pub enum DropDownCellMsg {
    Update(String)
}

impl Component for DropDownCell {
    type Message = DropDownCellMsg;
    type Properties = InteractiveCellProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            selected: DEFAULT_SELECTION_STRING,
            // correct_selection: ctx.props().text.middle.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DropDownCellMsg::Update(value) => { self.selected = value; true }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let text = ctx.props().text.clone();
        // let check_enabled = self.input_tracking.as_ref().map(|it: &InputTracking| it.check_table.is_some()).is_some();

        let dropdown_changed = ctx.link().callback(move |e: Event| {
            log_display("drop changed");
            let input: HtmlInputElement = e.target_unchecked_into();
            DropDownCellMsg::Update(input.value())
        });

        let checked_class =
            if ctx.props().check_mode && (self.selected != DEFAULT_SELECTION_STRING) {
                if self.selected == ctx.props().text.middle.clone() {
                    "correct_cell"
                } else {
                    "incorrect_cell"
                }
            } else {
                ""
            };

        // let selected = &ctx.props().options.first().map(|s: &String| s.clone()).unwrap_or(String::new());

        html! {
            <td class={checked_class}> { text.start }
                <select class={"table-input"} onchange={dropdown_changed.clone()} required={true}>
                    <option value={DEFAULT_SELECTION_STRING.clone()} disabled={true} selected={true} hidden={true}> {DEFAULT_SELECTION_STRING.clone()} </option>
                    { for (&ctx).props().options.iter().map(|o| { html! {
                        <option value={o.clone()}>{o}</option>
                    } }) }
                </select>
            { text.end } </td>
        }
    }

}

impl DropDownCell {

}