use web_sys::{HtmlInputElement, MouseEvent};
use yew::{Component, Context, Html, html, Properties};
use yew::prelude::*;
use std::collections::HashMap;

use crate::{log, log_display, log_js, get_text_width};
use crate::contexts::table::{InputTracking, Location};
use crate::contexts::{TriSplit, ThemeKind};

#[derive(Properties, PartialEq)]
pub struct SpoilerCellProps {
    pub theme: ThemeKind,
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
        let mut td_class =
            if ctx.props().do_fading.is_some() && self.spoiled {
                ctx.props().theme.css_class_themed("fade-in")
            } else {
                ctx.props().class.clone()
            };

        return html! {
            <td class={td_class} onmousedown={onclick.clone()}> { text.start } <span class={spoil_class} onmousedown={onclick}> { text.middle } </span> { text.end } </td>
        }
    }

}

pub(crate) const DEFAULT_SELECTION_STRING: String = String::new();

#[derive(Properties, PartialEq)]
pub struct DropDownCellProps {
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
    type Properties = DropDownCellProps;

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

        let checked_class = check_input(ctx.props().check_mode, self.selected.clone(), ctx.props().text.middle.clone());

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

pub struct TypeFieldCell {
    pub content: String,
    pub width: i32,
}

#[derive(Properties, PartialEq)]
pub struct TypeFieldCellProps {
    pub class: String,
    pub text: TriSplit,
    pub check_mode: bool,
    pub size: i32,
}

pub enum TypeFieldCellMsg {
    Update(String, i32)
}

impl Component for TypeFieldCell {
    type Message = TypeFieldCellMsg;
    type Properties = TypeFieldCellProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            content: DEFAULT_SELECTION_STRING,
            width: 20,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TypeFieldCellMsg::Update(value, width) => {
                if self.content != value {
                    self.content = value;
                    self.width = width;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text = ctx.props().text.clone();

        let paste = ctx.link().callback(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            log_display("p");
            let text_width: i32 = 0; // get_text_width(input.target);
            // input.set_length()

            TypeFieldCellMsg::Update(input.value(), text_width)
        });


        let content_changed = ctx.link().callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            log_display("c");
            let text_width: i32 = 0; // get_text_width(input); // TODO

            TypeFieldCellMsg::Update(input.value(), text_width)
        });

        // let width: String = format!("{}px", self.width + 4);

        let checked_class = check_input(ctx.props().check_mode, self.content.clone(), ctx.props().text.middle.clone());

        // TODO lengthen fields when typed into - https://jsfiddle.net/drq0nz6j/
        let class = ctx.props().class.clone();

        let size = ctx.props().size.to_string();

        return html! {
            <td class={checked_class}> { text.start }
                <input type="text" class={class} oninput={content_changed} onpaste={paste} size={size} /> { text.end } // onchange will wait until cell unfocused
            </td>
        }
    }

}

fn check_input(check_mode: bool, content: String, answer: String) -> &'static str {
    if check_mode {
        let content = convert_iso_shorthand(content);

        if content.len() == 0 {
            return ""; // content == DEFAULT_SELECTION_STRING
        }

        let content_bytes = content.as_bytes();

        let mut content_start: usize = 0;
        while content_bytes[content_start] == (32 as u8) {
            content_start += 1;
            if content_start == content_bytes.len() {
                return "";
            }
        }

        let mut content_end: usize = content_bytes.len() - 1;
        while content_bytes[content_end] == (32 as u8) {
            content_end -= 1;
        }

        // if content_start >= content_end + 1 { // " content " == DEFAULT_SELECTION_STRING
        //     return "";
        // }
        let answer_bytes = answer.as_bytes();
        if content_end + 1 - content_start != answer_bytes.len() {
            return "incorrect_cell";
        }
        let mut i: usize = 0;
        while i < answer_bytes.len() {
            if content_bytes[i + content_start] != answer_bytes[i] {
                return "incorrect_cell";
            }
            i = i + 1;
        }
        return "correct_cell";

    } else {
        ""
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

fn convert_iso_shorthand(mut shorthand: String) -> String {
    for (from, to) in &ISO_MAP {
        shorthand = shorthand.replace(from, to.clone());
    }
    shorthand
}
