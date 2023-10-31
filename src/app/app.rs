extern crate console_error_panic_hook;

use std::panic;

use percent_encoding::percent_decode_str;
use stylist::yew::{Global, styled_component};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{get_lessons_json, log_dbg, log_display, log_str};
use crate::contexts::{Exercise, ExerciseCategory, ExerciseComponent, Exercises, Lesson,
                      LessonsProvider, ThemeContext, ThemeProvider, Toolbar, use_lessons, use_theme, html_page};

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
    RedirectToLesson { path: String },

    #[at("/pali/lesson/:path/exercise/404")]
    RedirectToLesson2 { path: String },

    #[at("/pali/category/:category")]
    ExerciseCategory { category: ExerciseCategory },

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
    yew::Renderer::<App>::new().render();
}

#[styled_component(App)]
pub fn app() -> Html {
    return html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

pub fn empty_html() -> Html {
    return html! {};
}

#[derive(Properties, PartialEq, Clone)]
pub struct DefaultPageProps {
    pub toolbar: Html,
    pub main_content: Html
}

#[styled_component(ThemedPage)]
pub fn themed_content(props: &DefaultPageProps) -> Html {

    return html! {
            <DefaultPage ..props.clone() />
    }
}

#[styled_component(DefaultPage)]
pub fn content(props: &DefaultPageProps) -> Html {
    let theme: ThemeContext = use_theme();

    let main_class2 = css!{
       r#"background-color: ${bg_c};
                   width: 80vw;
                   max-width: 800px;
                   font-size: 20px;
                   min-height: calc(100% - 140px);
                   padding-top: 65px;
                   padding-bottom: 75px;
                "#, bg_c = theme.content_background_color.clone(), };

    return html! {
        <>

            // Global Styles can be applied with <Global /> component.
            <Global css={css!(
                r#"
                    html, body {
                        background-color: ${bg};
                        color: ${ft};
                    }

                    a:link, a:visited {
                        color: ${l};
                    }

                "#,
                bg = theme.default_background_color.clone(),
                ft = theme.font_color.clone(),
                l = theme.link_color.clone(),
            )} />

        { props.toolbar.clone() }
        <div class={css!(
        r#"width: 100vw;
           height: 100vh;
           display: flex;
           justify-content: center;
           align-items: baseline;
           padding-bottom: 25px;
           "#)}>
            <div class={classes!("main", main_class2)}>

                { props.main_content.clone() }

            </div>
        </div>
        </>
    }
}

pub fn content_from_toolbar(toolbar: Html, main_content: Html) -> Html {
    return html! { <ThemedPage toolbar={toolbar} main_content={main_content} /> }
}

pub fn content_titled(title: String, return_route: Option<Route>, main_content: Html) -> Html {
    content_from_toolbar(html! { <Toolbar name={title} return_route={return_route} /> }, main_content)
}

pub fn content_from(main_content: Html) -> Html {
    content_from_toolbar(html! { <Toolbar /> }, main_content)
}

fn switch(route: Route) -> Html {
    return html! {
        <LessonsProvider>
            <ThemeProvider>
                <SwitchLessons route={route} />
            </ThemeProvider>
        </LessonsProvider>
    }
}

#[derive(Properties, PartialEq)]
pub struct SwitchLessonsProps {
    pub route: Route,
}

#[styled_component(SwitchLessons)]
pub fn switch_with_lessons(props: &SwitchLessonsProps) -> Html {
    let lessons_ctx = use_lessons();
    let lessons = lessons_ctx.get_lessons();

    match props.route.clone() {
        Route::RedirectFromHome => html! { <Redirect<Route> to={Route::Overview} /> },
        Route::Overview => content_titled(String::from("Overview"), None, html! { <>
            <div class="info">
                <span>{"This is an interactive format which models "}</span>
                <a class="linked" target="_blank" href="https://archive.org/details/A.K.WarderPali/A.%20K.%20Warder%20Pali/mode/1up">{"Introduction To Pali by A.K. Warder."}</a>
                <br/>
                <br/>
                <span>{"This may not necessarily be correct, so please tell me any errors by messaging me "}</span>
                <a class="linked" href="https://discourse.suttacentral.net/u/bran">{"here"}</a>
                <span>{r#" or even any suggestions at all.
                            This is still a work-in-progress, so not every lesson is necessarily complete.
                            You can use this to memorize vocab, familiazize yourself, or quiz knowledge to track progression.
                            You should firstly look through the tutorial and options."#}</span>
                <br/>
                <h2 class="linked" > <Link<Route> to={Route::Lessons}>{ "View Lessons" }</Link<Route>> </h2>
                <br/>
                <span>{"I'll keep this "}</span>
                <a class="linked" target="_blank" href="https://github.com/Branzz/pali-course">{"open source"}</a>
                <span>{". The lessons are stored in an intuitive "}</span>
                <a class="linked" target="_blank" href="https://github.com/Branzz/pali-course/blob/master/src/main.js#L66">{"json format"}</a>
                <span>{", so it would be easy to contribute to that. Most of this isn't hard-coded, so one could clone this and use the format for learning anything else."}</span>
                <br/>
                <h2> { "Features" } </h2>
            </div>
            <div class="flex-surround">
                <ul class="boxxy">
                    <h3> { "Completed" } </h3>
                    <li> { "Framework for lesson creation" } </li>
                    <li> { "Exercise modes" } </li>
                    <li> { "Dark/Light theme" } </li>
                </ul>
                <ul class="boxxy">
                    <h3> { "In-progress" } </h3>
                    <li> { "Lessons 6+" } </li>
                    <li> { "Reveal-by-letter mode" } </li>
                </ul>
                <ul class="boxxy">
                    <h3> { "Planned / Other ideas" } </h3>
                    <li> { "Verb root meanings exercise" } </li>
                    <li> { "Multiple answers in one cell" } </li>
                    <li> { "Shuffle rows" } </li>
                    <li> { "Show the lesser definitions" } </li>
                </ul>
            </div>
            <h3> <Link<Route> to={Route::LearningResources}>{ "Other Resources" }</Link<Route>></h3>
       </> }),
        Route::LearningResources => content_titled(String::from("Resources"), Some(Route::Overview), html! { <>
            <div class="info">
                <div class="flexer"><p class="spaced">{ "Some links I have compiled along with some things I have made" }</p></div>
                <h3> <a target="_blank" href={ "https://archive.org/details/A.K.WarderPali/A.%20K.%20Warder%20Pali/mode/1up" }>{"Warder"}</a> </h3>
                <h3> <a target="_blank" href={ "https://app.memrise.com/course/910937/pali-ak-warder-vocabulary/" }>{"Similar Site"}</a> </h3>
                <h3> <a target="_blank" href={ "https://www.youtube.com/@BAUSChuangYenMonastery" }>{"Chuang Yen"}</a> </h3>
                <h3> <a target="_blank" href={ "https://www.ancient-buddhist-texts.net/Textual-Studies/Grammar/Guide-to-Pali-Grammar.htm" }>{"Rigid grammar guide"}</a> </h3>
                <h3> <a target="_blank" href={ "https://www.digitalpalireader.online/_dprhtml/index.html" }>{"Digital Pali Reader"}</a> </h3>
                <h3> <a target="_blank" href={ "https://audtip.org/misc/paligor/ref/index.html" }>{"Verb roots"}</a> </h3>
               <h3 class="spaced"> <a target="_blank" href={ "https://www.clearmountainmonastery.org/2020/08/01/article-a-fun-way-to-memorize-long-dhamma-with-a-special-focus-on-the-dhammapada/" }>{"Memorizing"}</a> </h3>
            </div>
            <div class="flexer"><p>{ "Chart of sounds in the mouth (bottom ones don't exist)" }</p></div>
            <div class="centered preserved"> <img src="/assets/phoen.png" /> </div>
            <div class="centered preserved"> <img src="/assets/sandhi.png" /> </div>
        </> }),
        Route::Lessons => {
            content_titled(String::from("Lessons"), Some(Route::Overview), html! { <>
                <br/>
                <div class="listed-info wide-text">
                    { for lessons.lessons.iter().map(|lesson| html! {
                        <Link<Route> to={Route::Lesson {path: lesson.path.clone()}}> { lesson.name.clone() } </Link<Route>>
                    }) }
                <br/>
                <h3> { "Categories" } </h3>
                    { for ExerciseCategory::iterator().map(|category| html! {
                        <Link<Route> to={Route::ExerciseCategory {category: category.clone()}}> { category.to_proper_string() } </Link<Route>>
                    }) }
                </div>
            </>})
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
            let lesson: Lesson = (*lessons.lessons.get(lesson_position).unwrap()).clone();

            let prev_route = prev_path.map(|path| Route::Lesson { path });
            let next_route = next_path.map(|path| Route::Lesson { path });

            content_from_toolbar(
                html! {
                    <Toolbar name={lesson.name} return_route={return_route} prev_route={prev_route} next_route={next_route}/>
                },
                html! {
                    <Exercises lesson_path={Some(path)} exercises={lesson.exercises}/>
                }
            )
        },
        Route::ExerciseCategory { category } => {
            let exercises: Vec<Exercise> = lessons.lessons.iter()
                .flat_map(|l: &Lesson| l.exercises.clone())
                .filter(|e: &Exercise| e.categories.as_ref().map(|cs: &Vec<ExerciseCategory>| cs.contains(&category)).unwrap_or(false))
                .collect();
            // let category_lesson = Lesson {
            //     name: category.to_string(),
            //     path: category.to_string(),
            //     exercises
            // };
            content_titled(category.to_proper_string(), Some(Route::Lessons), html! {
                <Exercises lesson_path={category.to_string()} exercises={exercises}/>
            })
        }
        Route::Exercise { lesson_path, exercise_path } => {
            html_page(lessons, lesson_path, exercise_path)
        },
        Route::NotFound => content_titled("404".to_string(), Some(Route::Overview), html! {
            <h1>{ "Not found" }</h1>
        }),
    }

}
