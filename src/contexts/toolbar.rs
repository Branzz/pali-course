#![allow(unused)]

use std::borrow::BorrowMut;
use std::ops::Deref;
use std::rc::Rc;

use stylist::css;
use stylist::yew::{Global, styled_component};
use yew::{Component, Context, Html, html, MouseEvent, Properties, use_context, use_state, UseStateHandle};
use yew::Callback;
use yew::context::ContextHandle;
use yew::html::Scope;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::prelude::*;

use crate::{html_if_some, log_js, log_str, ProviderProps};
use crate::app::Route;
use crate::contexts::{ThemeContext, ThemeKind, ThemeSwitcher, use_theme};

#[derive(Properties, PartialEq)]
pub struct ToolbarProps {
    pub name: Option<String>,
    pub return_route: Option<Route>,
    pub prev_route: Option<Route>,
    pub next_route: Option<Route>,
}
pub static TOOLBAR_HEIGHT: &str = "40px";

#[styled_component(Toolbar)]
pub fn toolbar(props: &ToolbarProps) -> Html {
    let theme_context: ThemeContext = use_theme();
    let theme = theme_context.kind();
    let background_color = theme_context.toolbar_background_color.clone();
    let font_color = theme_context.font_color.clone(); // String::from("white");

    let top_button = theme_context.kind().css_class_themed("top-button");
        // let navigator = use_navigator().unwrap();
    let return_html = props.return_route.clone().map(|return_route| html! {
                <div class={css!(r#" margin-left: 0px; line-height: 34px; "#)}>
                    <div class={top_button.clone()}>
                        <Link<Route> to={return_route}>
                            { "🡴" }
                        </Link<Route>>
                    </div>
                </div>
    }).unwrap_or(html! {<div> </div>} ); // hidden element to align flex parent

    let prev = props.prev_route.clone().map(|prev| html! {
                <div class={css!(r#"line-height: 33px; padding-right: 12px; "#)}>
                    <div class={top_button.clone()}> // can't put element around link or it will refresh page
                        // <img src="/arrow.png" width="28" height="28" class={css!("padding-top: 2px;")}/>
                        <Link<Route> to={prev}>
                            { "⮜" }
                        </Link<Route>>
                    </div>
                </div>
    }).unwrap_or(html! {<div> </div>} );

    let top = props.name.clone().map(|name| html! {
                <div class={css!(r#" width: min-content; text-align: center; margin-top: 3px; width: 1600px; color: #eae4d7; "#)}> // twice width as content
                    { name }
                </div>
    }).unwrap_or(html! {<div> </div>} );

    let next = props.next_route.clone().map(|next| html! {
                <div class={css!(r#"line-height: 33px; padding-left: 12px; "#)}>
                    <div class={top_button}>
                        <Link<Route> to={next}>
                            { "⮞" }
                        </Link<Route>>
                    </div>
                </div>
        }
    ).unwrap_or(html! {<div> </div>} );

    html! {
    <>
      <div class={css!(
            r#"
            width: 100vw;
            height: ${h};
            background-color: ${bg_c};
            color: ${fc};
            position: absolute;
            "#, h = TOOLBAR_HEIGHT, bg_c = background_color, fc = font_color
      )}>
        <div class={css!(
            r#"
            display: flex;
            justify-content: center;
            font-size: 24pt;
            line-height: 32px;
            "#
        )} >
            <div class="filler-left flex-spread">
                { return_html }
                { prev }
            </div>
                { top }
            <div class="filler-right flex-spread">
                { next }
                <ThemeSwitcher />
            </div>
        </div>
      </div>
    </>
    }
}
