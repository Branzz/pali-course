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
use yew_router::prelude::*;
use yew::virtual_dom::VNode;

use crate::app::Route;
use crate::{log, log_js, ProviderProps, html_if_some};
use crate::contexts::{RunState, ThemeContext, ThemeKind, ThemeSwitcher, use_theme};
use crate::contexts::runner::{RunnerAction, RunStateType, StateAction};

#[styled_component(RunnerProvider)]
pub(crate) fn runner_provider(props: &ProviderProps) -> Html {
    let run_state: ToolbarContext = use_reducer(|| RunState { run_state: RunStateType::B }); // TODO _eq

    html! {
        <ContextProvider<ToolbarContext> context={run_state}>
            {props.children.clone()}
        </ContextProvider<ToolbarContext>>
    }
}

pub(crate) type ToolbarContext = UseReducerHandle<RunState>;

impl Reducible for RunState {
    type Action = RunnerAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_state = self.state_act(action);
        match next_state {
            // RunState { run_state: RunStateType::do_nothing } => self.clone(), // it breaks when users click Nothing buttons otherwise
            _ => next_state.into()
        }
    }
}


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
    let toolbar_context = use_context::<ToolbarContext>().unwrap();
    let theme_context: ThemeContext = use_theme();
    let theme = theme_context.kind();
    let background_color = theme_context.toolbar_background_color.clone();

    // let props1 = yew::props!(ToolbarButtonProps { theme: theme.clone(), tb_ctx: toolbar_context.clone(), icon_name: "1", state: RunStateType::A });
    // let props2 = yew::props!(ToolbarButtonProps { theme: theme.clone(), tb_ctx: toolbar_context.clone(), icon_name: "2", state: RunStateType::B });
    // let props3 = yew::props!(ToolbarButtonProps { theme: theme.clone(), tb_ctx: toolbar_context.clone(), icon_name: "3", state: RunStateType::C });

    let return_html = props.return_route.clone().map(|return_route| html! {
                <div class={css!(r#" top: 0px; margin: 2px;
                                     left: 10px; "#)}>
                    <div class={"top-button"}>
                        <Link<Route> to={return_route}>
                            { "↩" }
                        </Link<Route>>
                    </div>
                </div>
    }).unwrap_or(html! {<div> </div>} ); // hidden element to align flex parent

    let prev = props.prev_route.clone().map(|prev| html! {
                <div class={css!(r#" "#)}>
                    <div class={"top-button"}>
                        <Link<Route> to={prev}>
                            { "<" }
                        </Link<Route>>
                    </div>
                </div>
    }).unwrap_or(html! {<div> </div>} );

    let top = props.name.clone().map(|name| html! {
                <div class={css!(r#" width: 25%; text-align: center; "#)}>
                    { name }
                </div>
    }).unwrap_or(html! {<div> </div>} );

    let next = props.next_route.clone().map(|next| html! {
                <div class={css!(r#" "#)}>
                    <div class={"top-button"}>
                        <Link<Route> to={next}>
                            { ">" }
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
            "#, h = TOOLBAR_HEIGHT, bg_c = background_color,
      )}>
        <div class={css!(
            r#"
            display: flex;
            justify-content: space-between;
            font-size: 24pt;
            "#
        )} >
            { return_html }
            { prev }
            { top }
            { next }
            <ThemeSwitcher />
        </div>
      </div>
    </>
    }
}

#[styled_component(NamedToolbar)]
pub fn named_toolbar() -> Html {
    let toolbar_context = use_context::<ToolbarContext>().unwrap();
    let theme_context: ThemeContext = use_theme();
    let theme = theme_context.kind();
    let background_color = theme_context.toolbar_background_color.clone();

    html! {
    <>
      <div class={css!(
            r#"
            width: 100vw;
            height: ${h};
            background-color: ${bg_c};
            "#, h = TOOLBAR_HEIGHT, bg_c = background_color,
      )}>
      </div>
    </>
    }
}

pub(crate) struct ToolbarButton;

#[derive(Clone, PartialEq, Debug, Properties)]
pub(crate) struct ToolbarButtonProps {
    pub theme: ThemeKind,
    pub tb_ctx: ToolbarContext,
    pub icon_name: String,
    pub state: RunStateType,
}

impl Component for ToolbarButton {
    type Message = (); // if shift key pressed
    type Properties = ToolbarButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { }
    }

    #[allow(unused_variables)]
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let theme: ThemeKind = ctx.props().theme.clone();
        let tb_ctx: ToolbarContext = ctx.props().tb_ctx.clone();
        let background_color = theme.current().toolbar_background_color.clone();
        let icon_name = ctx.props().icon_name.clone();
        let disabled = icon_name.clone().as_str().ends_with("grayed");
        let hover_background_color = String::from(if disabled { "transparent" } else { &theme.current().hover_color });
        let state = ctx.props().state.clone();
        let mut rgb: [u8; 3] = [30, 30, 30];
        rgb[tb_ctx.clone().index()] += 105;
        rgb[state.clone().index()] += 75;
        let color_str= format!("rgb({:?}, {:?}, {:?})", rgb[0], rgb[1], rgb[2]);

        let callback: Callback<MouseEvent> = Callback::from(move |e: MouseEvent| {
            tb_ctx.dispatch( RunnerAction::new(StateAction::To(RunState { run_state: state })) );
        });
        // use values from MouseEvent, like if it's a shift click, to do more actions

        html! {
          <>
            <button class={css!(
              r#"
                 height: calc(2.5 * ${h} / 3);
                 width: calc(2.5 * ${h} / 3);
                 margin-right: calc(${h} / 4);
                 margin-top: calc(${h} / 12);
                 background-size: calc(5 * ${h} / 8);
                 background-repeat: no-repeat;
                 background-position: center;
                 background-image: url("/assets/toolbar_icons/${i}.png");
                 border: dashed lightslategray 0px;
                 border-radius: 2px;
                 transition: all .25s ease-in-out;
                 background-color: ${c};

                 &:hover {
                     background-color: ${hbc};
                 }
              "#, i = icon_name, hbc = hover_background_color, h = TOOLBAR_HEIGHT, c = color_str
              )} onclick={callback} >
            </button>
          </>
        }
    }

}
