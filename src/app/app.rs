extern crate console_error_panic_hook;

use std::panic;

use stylist::yew::{Global, styled_component};
use yew::prelude::*;
use yew::props;

use crate::contexts::{RunnerProvider, ThemeContext, ThemeKind, ThemeProvider, ToolbarContext, use_theme};
use crate::contexts::runner::RunStateType;
use crate::contexts::toolbar::{TOOLBAR_HEIGHT, Toolbar};
use crate::log;

pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    set_event_bubbling(false);
    yew::Renderer::<Root>::new().render();
}

#[styled_component(Root)]
pub fn root() -> Html {
    (html! {
        <ThemeProvider>
            <App />
        </ThemeProvider>
    }) as Html
}

#[styled_component(App)]
fn app() -> Html {
    let theme: ThemeContext = use_theme();
    html! {
        <>
            // Global Styles can be applied with <Global /> component.
            <Global css={css!(
                r#"
                    html, body {
                        font-family: "Skolar Sutta Web", serif;
                        padding: 0;
                        margin: 0;
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        flex-direction: column;
                        background-color: ${bg};
                        color: ${ft_color};
                        overflow: hidden;
                    }
                "#,
                bg = theme.default_background_color.clone(),
                ft_color = theme.font_color.clone(),
            )} />
            <RunnerProvider>
                <Toolbar />
                <div class={css!(
                        r#"
                        height: calc(100vh - ${th});
                        width: 100vw;
                        "#, th = TOOLBAR_HEIGHT
                )} >
                    <Content />
                </div>

            </RunnerProvider>

        </>
    }
}

/*
 boiler plate:

<div class={css!(r#""#)}>
</div>

 */



#[styled_component(Content)]
pub fn content() -> Html {
    let theme: ThemeContext = use_theme();
    html! {
      <>
        <div class={css!(
             r#"background-color: ${bg_c};
                position: absolute;
                height: 80%;
                width: 70%;
                left: 10%;
                padding: 5%;
             "#, bg_c = theme.content_background_color.clone(),
        )}>
            <div class={css!(
                r#"
                    font-size: 18px;
                    overflow-y: auto;
                "#,
            )}>
                <Comp />
            </div>
        </div>
      </>
    }
}

#[styled_component(Comp)]
fn comp() -> Html {
    let _ = 0;

    html! {
      <>
        <div class={css!(
            r#"
            "#,
        )}>
            <SampleHOC/>
        </div>
      </>
    }
}

#[function_component]
pub fn SampleHOC() -> Html {
    let theme = use_theme().kind();
    let toolbar_context: ToolbarContext = use_context::<ToolbarContext>().unwrap();
    let state = toolbar_context.index().to_string();

    let props: SampleProps = props! {
        SampleProps{ theme: theme, tb_state: state }
    };
    html! {
        <Sample ..props />
    }
}

#[derive(Properties, PartialEq)]
struct SampleProps {
    pub theme: ThemeKind,
    pub tb_state: String,
}

struct Sample;

struct SampleMsg;

impl Component for Sample {
    type Message = SampleMsg;
    type Properties = SampleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let our_str = "font color: ".to_owned() + ctx.props().theme.clone().current().font_color.as_str()
                        + " | state: " + &ctx.props().tb_state;

        html! {
           { our_str }
        }
    }

}
