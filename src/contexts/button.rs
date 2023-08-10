use yew::{Context, Component, Html, html, Properties};
use crate::contexts::TriSplit;
use crate::log;
use web_sys::MouseEvent;

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

            log("clicked");
            SpoilerCellMsg::FlipState
        });

        let spoil_class = if self.spoiled { "spoiler_button invisible" } else { "spoiler_button visible" };
        let text = ctx.props().text.clone();

        html! {
            <td class={"interactive"} onmousedown={onclick.clone()}> { text.start } <span class={spoil_class} onmousedown={onclick}> { text.middle } </span> { text.end } </td>
        }
    }

}
