extern crate console_error_panic_hook;

use std::panic;

use stylist::yew::{Global, styled_component};
use yew::prelude::*;
use yew::props;
use gloo_net::http::Request;
use yew_router::prelude::*;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen::from_value;

use crate::contexts::toolbar::{TOOLBAR_HEIGHT};
use crate::contexts::{RunnerProvider, ThemeContext, ThemeKind, ThemeProvider, ToolbarContext,
                      Toolbar, NamedToolbar, use_theme, ExerciseComponent,  ExerciseComponentProps,
                      ExerciseMode, Lessons, Lesson, Exercise, Exercises};
use crate::{get_lessons_json, log_str};
use percent_encoding::percent_decode_str;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    RedirectFromHome,

    #[at("/pali")]
    Overview,

    #[at("/pali/resources")]
    LearningResources,

    #[at("/pali/lessons")]
    Lessons,

    #[at("/pali/lesson/:path")]
    Lesson { path: String },

    #[at("/pali/lessons/:path")]
    // #[at("/pali/exercises/:path")]
    RedirectToLesson { path: String },

    #[at("/pali/lesson/:path/exercise/404")]
    // #[at("/pali/exercises/:path")]
    RedirectToLesson2 { path: String },

    #[at("/pali/lesson/:lesson_path/exercise/:exercise_path")]
    Exercise { lesson_path: String, exercise_path: String },

    // #[at("/pali/lessons/:exercise_path")]
    // RedirectToExercise { path: String },

    #[not_found]
    #[at("/404")]
    NotFound,
}

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
                        background-color: ${bg};
                        color: ${ft_color};
                    }
                "#,
                bg = theme.default_background_color.clone(),
                ft_color = theme.font_color.clone(),
            )} />

            <RouteBranching />

        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct DefaultPageProps {
    pub toolbar: Html,
    pub main_content: Html
}

#[styled_component(DefaultPage)]
pub fn content(props: &DefaultPageProps) -> Html {
    let theme: ThemeContext = use_theme();

    return html! {
        <RunnerProvider>
            { props.toolbar.clone() }
            <div class={css!(r#"height: calc(100vh - ${th}); width: 100vw;"#, th = TOOLBAR_HEIGHT)}>
                <div class={css!(
                    r#"background-color: ${bg_c};
                       position: relative;
                       height: calc(100% - 80px);
                       width: 70%;
                       left: 10%;
                       padding: 5%;
                       padding-top: 2%;
                       padding-bottom: 20px;
                       font-size: 20px;
                       overflow-y: auto;
                    "#, bg_c = theme.content_background_color.clone(),
                )}>

                    { props.main_content.clone() }

                </div>
            </div>
        </RunnerProvider>
    }
}

pub fn content_from_toolbar(toolbar: Html, main_content: Html) -> Html {
    return html! { <DefaultPage toolbar={toolbar} main_content={main_content} /> }
}

pub fn content_titled(title: String, return_route: Option<Route>, main_content: Html) -> Html {
    content_from_toolbar(html! { <Toolbar name={title} return_route={return_route} /> }, main_content)
}

pub fn content_from(main_content: Html) -> Html {
    content_from_toolbar(html! { <Toolbar /> }, main_content)
}

fn switch(routes: Route) -> Html {
    let mut lessons = from_value::<Lessons>(get_lessons_json()).expect("couldn't load json");

    match routes {
        Route::RedirectFromHome => html! { <Redirect<Route> to={Route::Overview} /> },
        Route::Overview => content_titled(String::from("Overview"), None, html! { <>
            <div class={"info"}>
                // <h1>{ "a" }</h1>
                <span>{"This is an interactive format from "}</span>
                <a href="https://archive.org/details/A.K.WarderPali/A.%20K.%20Warder%20Pali/mode/1up">{"Introduction To Pali by A.K. Warder."}</a>
                <h2> <Link<Route> to={Route::Lessons}>{ "View Lessons" }</Link<Route>> </h2>
                <span>{"This may not necessarily be correct, so please tell me any errors "}</span>
                <a href="https://discourse.suttacentral.net/u/bran">{"here"}</a>
                <span>{r#" or even any suggestions at all.
                            This is definitely a work-in-progress, so not every lesson is as full as I'd like.
                            You can use this to memorize vocab, familiazize yourself, or quiz knowledge to track progression.
                            You should firstly look through the tutorial and options."#}</span>
                <h3> <Link<Route> to={Route::LearningResources}>{ "Other Resources" }</Link<Route>></h3>
                <span>{"I'll keep this "}</span>
                <a href="https://github.com/Branzz/pali-course">{"open source"}</a>
                <span>{". If you'd like to contribute somehow, this was made in a lesson known framework, Yew (React-like) in Rust, transpiled to WebAssembly."}</span>
                <br/>
                <br/>
                <span>{"The lessons are stored in an intuitive "}</span>
                <a href="https://github.com/Branzz/pali-course/blob/master/src/main.js#L67">{"json format"}</a>
                <span>{", however, so it would be easy to add to that. Most of this isn't hard-coded, so one could clone this and easily use the format for learning anything else."}</span>
                <br/>
                <h2> { "Features" } </h2>
            </div>
            <div class={"flex-surround"}>
                <ul class={"boxxy"}>
                    <h3> { "Completed" } </h3>
                    <li> { "Framework for lesson creation" } </li>
                    // <li> { "" } </li>
                </ul>
                <ul class={"boxxy"}>
                    <h3> { "In-progress" } </h3>
                    <li> { "Tutorial, Lessons 1-2" } </li>
                    <li> { "Display modes" } </li>
                    <li> { "Dark/Light theme" } </li>
                    // <li> { "." } </li>
                </ul>
                <ul class={"boxxy"}>
                    <h3> { "Planned / other ideas" } </h3>
                    <li> { "Pāli letter paste buttons" } </li>
                    <li> { "Lessons 3+" } </li>
                    <li> { "Mobile friendly (mouse-hover, reactive)" } </li>
                </ul>
            </div>
        </> }),
        Route::LearningResources => content_titled(String::from("Resources"), Some(Route::Overview), html! { <>
            <div class={"info"}>
                <p class={"spaced"}>{ "Some links I have compiled along with some things I have made" }</p>
                <h3> <a href={ "https://archive.org/details/A.K.WarderPali/A.%20K.%20Warder%20Pali/mode/1up" }>{"Warder"}</a> </h3>
                <h3> <a href={ "https://www.ancient-buddhist-texts.net/Textual-Studies/Grammar/Guide-to-Pali-Grammar.htm" }>{"Rigid grammar guide"}</a> </h3>
                <h3> <a href={ "https://www.digitalpalireader.online/_dprhtml/index.html" }>{"Digital Pali Reader"}</a> </h3>
                <h3 class={"spaced"}> <a href={ "https://www.clearmountainmonastery.org/2020/08/01/article-a-fun-way-to-memorize-long-dhamma-with-a-special-focus-on-the-dhammapada/" }>{"Memorizing"}</a> </h3>
            </div>
            <div class={"centered"}> <img src="/phoen.png" /> </div>
            <div class={"centered"}> <img src="/sandhi.png" /> </div>
        </> }),
        Route::Lessons => {
            content_titled(String::from("Lessons"), Some(Route::Overview), html! {
                <div class={"listed-info wide-text"}>
                    { for lessons.lessons.iter().map(|lesson| html! {
                        <Link<Route> to={Route::Lesson {path: lesson.path.clone()}}> { lesson.name.clone() } </Link<Route>>
                    }) }
                </div>
            })
        },
        Route::RedirectToLesson  { path } => html! { <Redirect<Route> to={Route::Lesson {path: path}} /> },
        Route::RedirectToLesson2 { path } => html! { <Redirect<Route> to={Route::Lesson {path: path}} /> },
        Route::Lesson { path } => {
            let lesson_position_opt = lessons.lessons.iter().position(|l: &Lesson| l.path == path);
            if lesson_position_opt.is_none() {
                return content_from( html! { <> <h1> { "Unknown lesson" } </h1> <Link<Route> to={Route::Lessons}> {"Return"} </Link<Route>> </> } )
            }
            let lesson_position = lesson_position_opt.unwrap();

            let return_route = Some(Route::Lessons);
            let prev_path = if lesson_position == 0 {None} else {lessons.lessons.get(lesson_position - 1).map(|l: &Lesson| l.path.clone())};
            let next_path = lessons.lessons.get(lesson_position + 1).map(|l: &Lesson| l.path.clone());
            let lesson: Lesson = lessons.lessons.remove(lesson_position);

            let prev_route = prev_path.map(|path| Route::Lesson { path });
            let next_route = next_path.map(|path| Route::Lesson { path });

            content_from_toolbar(
                html! {
                    <Toolbar name={lesson.name} return_route={return_route} prev_route={prev_route} next_route={next_route}/>
                },
                html! {
                    <Exercises exercises={lesson.exercises}/>
                }
            )
        },
        Route::Exercise {lesson_path, exercise_path} => {
            let lesson_position_opt = lessons.lessons.iter().position(|l: &Lesson| l.path == lesson_path);
            if lesson_position_opt.is_none() {
                return content_from( html! { <> <h1> { "Unknown lesson" } </h1> <Link<Route> to={Route::Lessons} /> </> } )
            }
            let lesson_position = lesson_position_opt.unwrap();

            let mut lesson: Lesson = lessons.lessons.remove(lesson_position);
            let lesson_path = lesson.path;

            let decoded_exercise_path = percent_decode_str(exercise_path.as_str()).decode_utf8().unwrap();

            log_str(&*decoded_exercise_path.clone());

            let exercise_position_opt = lesson.exercises.iter().position(|e: &Exercise| e.effective_path() == decoded_exercise_path);
            if exercise_position_opt.is_none() {
                return content_from( html! { <> <h1> { "Unknown exercise" } </h1> <Link<Route> to={Route::Lesson {path: lesson_path.clone()}}> { "Return" } </Link<Route>> </> } )
            }
            let exercise_position = exercise_position_opt.unwrap();

            let (prev_lesson_path, prev_exercise_path): (Option<String>, Option<String>) =
                if exercise_position == 0 {
                    if lesson_position == 0 {
                        (None, None)
                    } else {
                        let l: Option<&Lesson> = lessons.lessons.get((lesson_position as i32 - 1) as usize);
                        (l.map(|l: &Lesson| l.path.clone()), l.map(|l: &Lesson| l.exercises.last()).flatten().map(|e| e.effective_path().clone()))
                    }
                } else {
                    (Some(lesson_path.clone()), lesson.exercises.get((exercise_position as i32 - 1) as usize).map(|e: &Exercise| e.effective_path().clone()))
                };

            let (next_lesson_path, next_exercise_path): (Option<String>, Option<String>) =
                if exercise_position == lesson.exercises.len() - 1 {
                    if lesson_position == lessons.lessons.len() - 1 {
                        (None, None)
                    } else {
                        let l: Option<&Lesson> = lessons.lessons.get(lesson_position);
                        (l.map(|l: &Lesson| l.path.clone()), l.map(|l: &Lesson| l.exercises.first()).flatten().map(|e| e.effective_path().clone()))
                    }
                } else {
                    (Some(lesson_path.clone()), lesson.exercises.get(exercise_position + 1).map(|e: &Exercise| e.effective_path().clone()))
                };


            let return_route = Some(Route::Lesson { path: lesson_path });
            let prev_route = prev_lesson_path.map(|lesson_path| prev_exercise_path.map(|exercise_path| {
                Some(Route::Exercise { lesson_path, exercise_path })
            }).unwrap_or(None)).unwrap_or(None);
            let next_route = next_lesson_path.map(|lesson_path| next_exercise_path.map(|exercise_path| {
                Some(Route::Exercise { lesson_path, exercise_path })
            }).unwrap_or(None)).unwrap_or(None);
            let exercise: Exercise = lesson.exercises.remove(exercise_position);

            content_from_toolbar(
                html! {
                    <Toolbar name={lesson.name} return_route={return_route} prev_route={prev_route} next_route={next_route}/>
                },
                html! {
                    <ExerciseComponent exercise={exercise}/>
                }
            )
        },
        Route::NotFound => content_from(html! {
            <h1>{ "404" }</h1>
        }),
    }
}

#[function_component(RouteBranching)]
fn routed() -> Html {
    return html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

pub fn empty_html() -> Html {
    return html! {};
}



//                            eof                            //



// #[styled_component(Comp)]
// fn comp() -> Html {
//     return html! {
//       <>
//         <div class={css!(
//             r#"
//             "#,
//         )}>
//             <SampleHOC/>
//         </div>
//       </>
//     }
// }
//
// // wrap users around Component
// #[function_component]
// pub fn SampleHOC() -> Html {
//     let theme = use_theme().kind();
//     let toolbar_context: ToolbarContext = use_context::<ToolbarContext>().unwrap();
//     let state = toolbar_context.index().to_string();
//
//     let props: SampleProps = props! {
//         SampleProps{ theme: theme, tb_state: state }
//     };
//     html! {
//         <Sample ..props />
//     }
// }
//
// #[derive(Properties, PartialEq)]
// pub struct SampleProps {
//     pub theme: ThemeKind,
//     pub tb_state: String,
// }
//
// pub struct Sample;
//
// pub struct SampleMsg;
//
// impl Component for Sample {
//     type Message = SampleMsg;
//     type Properties = SampleProps;
//
//     fn create(_ctx: &Context<Self>) -> Self {
//         Self { }
//     }
//
//     fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
//         true
//     }
//
//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let our_str = "font color: ".to_owned() + ctx.props().theme.clone().current().font_color.as_str()
//                         + " | state: " + &ctx.props().tb_state;
//
//         html! {
//            { our_str }
//         }
//     }
//
// }
