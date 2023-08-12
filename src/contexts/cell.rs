use web_sys::{HtmlInputElement, MouseEvent};
use yew::{Component, Context, Html, html, Properties};
use yew::prelude::*;
use std::collections::HashMap;

use crate::{log, log_display};
use crate::contexts::table::{InputTracking, Location};
use crate::contexts::TriSplit;

#[derive(Properties, PartialEq)]
pub struct SpoilerCellProps {
    pub class: String,
    pub text: TriSplit,
    pub do_fading: Option<()>,
}

pub struct SpoilerCell {
    spoiled: bool,
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
        let onclick = ctx.link().callback(move |_e: MouseEvent| {
            SpoilerCellMsg::FlipState
        });

        let spoil_class = if self.spoiled { "spoiler_button invisible" } else { "spoiler_button visible" };
        let text = ctx.props().text.clone();
        let mut td_class = ctx.props().class.clone();
        if ctx.props().do_fading.is_some() && self.spoiled {
            td_class.push_str(" fade-in");
        }

        return html! {
            <td class={td_class} onmousedown={onclick.clone()}> { text.start } <span class={spoil_class} onmousedown={onclick}> { text.middle } </span> { text.end } </td>
        }
    }

}

pub(crate) const DEFAULT_SELECTION_STRING: String = String::new();

#[derive(Properties, PartialEq)]
pub struct InteractiveCellProps {
    pub class: String,
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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected: DEFAULT_SELECTION_STRING,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DropDownCellMsg::Update(value) => { self.selected = value; true }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let text = ctx.props().text.clone();

        let dropdown_changed = ctx.link().callback(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            DropDownCellMsg::Update(input.value())
        });

        let checked_class =
            if ctx.props().check_mode && (self.selected != DEFAULT_SELECTION_STRING) {
                if self.selected == ctx.props().text.middle.clone()
                    || iso_shorthand_equals(&self.selected, &ctx.props().text.middle.clone()) {
                    "correct_cell"
                } else {
                    "incorrect_cell"
                }
            } else {
                ""
            };

        // let selected = &ctx.props().options.first().map(|s: &String| s.clone()).unwrap_or(String::new());
        let mut class = ctx.props().class.clone();
        class.push_str(" table-input");

        return html! {
            <td class={checked_class}> { text.start }
                <select class={class} onchange={dropdown_changed.clone()} required={true}>
                    <option value={DEFAULT_SELECTION_STRING.clone()} disabled={true} selected={true} hidden={true}> {DEFAULT_SELECTION_STRING.clone()} </option>
                    { for (&ctx).props().options.iter().map(|o| { html! {
                        <option value={o.clone()}>{o}</option>
                    } }) }
                </select>
            { text.end } </td>
        }
    }

}

// match the description in main.js
const ISO_MAP: [(&str, &str); 10] = [
    ("aa", "ā"),
    ("ii", "ī"),
    ("uu", "ū"),
    (".t", "ṭ"),
    (".d", "ḍ"),
    ("`n", "ṅ"),
    ("~n", "ñ"),
    (".n", "ṇ"),
    (".m", "ṃ"),
    (".l", "ḷ"),
];

fn iso_shorthand_equals(shorthand: &String, iso: &String) -> bool {
    let mut converted = shorthand.clone();
    for (from, to) in &ISO_MAP {
        converted = converted.replace(from, to.clone());
    }
    return &converted == iso;
}
