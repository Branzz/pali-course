use gloo_net::Error;
use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use stylist::yew::{Global, styled_component};
use wasm_bindgen_futures::spawn_local;
use yew::{Component, Context, Html, html};
use yew::prelude::*;

use crate::contexts::exercise::{Exercise, ExerciseComponent, ExerciseComponentProps};

// fn load_json() {
//     let lessons = use_state(|| vec![]);
//     {
//         // let lessons = lessons.clone();
//         use_effect_with_deps(move |_| {
//             // let lessons = lessons.clone();
//             spawn_local(async move {
//                 let fetched_lessons: Lessons = Request::get("lessons.js")
//                     .send()
//                     .await
//                     .unwrap()
//                     .json()
//                     .await
//                     .unwrap();
//                 lessons.set(fetched_lessons);
//             });
//             || ()
//         }, ());
//     }
// }

// pub fn get_lessons() -> Lessons {
//     Lessons { lessons: None }
// }

// pub fn get_lesson(name: String) -> Lesson {
//     // let name_str: &'static str = name.leak();
//     Lesson { name: name, exercises: sample_exercises() }
// }

// fn sample_exercises() -> Vec<Exercise> {
//     // vec![Exercise::new("abc".to_string(), "Yatonidānaṁ, bhikkhu, purisaṁ papañcasaññāsaṅkhā samudācaranti. Ettha ce natthi abhinanditabbaṁ abhivaditabbaṁ ajjhositabbaṁ. Esevanto rāgānusayānaṁ, esevanto paṭighānusayānaṁ, esevanto diṭṭhānusayānaṁ, esevanto vicikicchānusayānaṁ, esevanto mānānusayānaṁ, esevanto bhavarāgānusayānaṁ, esevanto avijjānusayānaṁ, esevanto daṇḍādānasatthādānakalahaviggahavivādatuvaṁtuvaṁpesuññamusāvādānaṁ. Etthete pāpakā akusalā dhammā aparisesā nirujjhantī.".to_string(),
//     //                 vec![vec!["aaaa|bbbb|cccc".to_string(), "|tha|".to_string(), "zzzz||".to_string()], vec!["taa||zzz".to_string(), "x|aaax|aa|h".to_string(), "aaa|aaa".to_string()]]),
//         // Exercise::new("Conjugate Bhuu", "",
//         //             &[&["person",   "singular",     "plural"],
//         //               &["1st",      "bhav|aami|",   "bhav|aama|"],
//         //               &["2nd",      "bhav|asi|",    "bhav|atha|"],
//         //               &["3rd",      "bhav|ati|",    "bhav|anti|"]])
//     ]
// }

#[derive(Properties, PartialEq)]
pub struct ExercisesProps {
    pub exercises: Vec<Exercise>
}

#[styled_component(Exercises)]
pub fn exercises(props: &ExercisesProps) -> Html {
    return html! {
        { for props.exercises.iter().map(|e| html! {
            <ExerciseComponent exercise={e.clone()} />
        }) }
    }
}



#[derive(PartialEq, Clone, Deserialize)]
pub struct Lesson {
    pub name: String,
    pub path: String,
    pub exercises: Vec<Exercise>
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct Lessons {
    pub lessons: Vec<Lesson>
}

// #[derive(Debug)]
// pub struct FetchServiceExample {
//     fetch_task: Option<FetchTask>,
//     lessons: Option<Lessons>,
//     // link: ComponentLink<Self>,
//     error: Option<String>,
// }
//
// impl FetchServiceExample {
//
//
//     fn view_iss_location(&self) -> Html {
//          match self.lessons {
//             Some(ref lessons0) => {
//                 return html! {
//                     <>
//                         <h1>{ lessons0.name }</h1>
//                         for lessons0.lessons.iter().map(|exercise| {
//                             exercise.get_html()
//                         })
//                     </>
//                 };
//             }
//             None => {
//                 return html! {
//                     html! {
//                         { "Loading lesson data..." }
//                     }
//                 };
//             }
//         }
//     }
//     fn view_fetching(&self) -> Html {
//         if self.fetch_task.is_some() {
//             return html! { <p>{ "Fetching data..." }</p> }
//         } else {
//             return html! { <p></p> }
//         }
//     }
//     fn view_error(&self) -> Html {
//         if let Some(ref error) = self.error {
//             return html! { <p>{ error.clone() }</p> }
//         } else {
//             return html! {}
//         }
//     }
// }
//
// #[derive(Debug)]
// pub enum Msg {
//     // GetLocation,
//     ReceiveResponse(Result<Lessons, Error>),
// }
//
// impl Component for FetchServiceExample {
//     type Message = Msg;
//     type Properties = ();
//
//     fn create(_ctx: &Context<Self>) -> Self {
//         // 1. build the request
//         let request = Request::get("lessons.js")
//             .body(Nothing)
//             .expect("Could not build request.");
//         // 2. construct a callback
//         let callback: Callback<Response, Msg> = Callback::from(|response: Response| Msg::ReceiveResponse(response.json()) );
//         // 3. pass the request and callback to the fetch service
//         // _ctx.with_subscriber()
//         let task = FetchService::fetch(request, callback).expect("failed to start request");
//         // 4. store the task so it isn't canceled immediately
//         Self {
//             fetch_task: Some(task),
//             lessons: None,
//             error: None
//         }
//         // we want to redraw so that the page displays a 'fetching...' message to the user
//         // so return 'true'
//
//
//     }
//
//
//     // fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
//     //     Self {
//     //         fetch_task: None,
//     //         iss: None,
//     //         link,
//     //         error: None,
//     //     }
//     // }
//
//     // fn change(&mut self, _props: Self::Properties) -> bool {
//     //     false
//     // }
//
//     fn update(&mut self, msg: Self::Message) -> bool {
//         use Msg::*;
//
//         match msg {
//             GetLocation => {
//             }
//             ReceiveResponse(response) => {
//                 match response {
//                     Ok(location) => {
//                         self.lessons = Some(location);
//                     }
//                     Err(error) => {
//                         self.error = Some(error.to_string())
//                     }
//                 }
//                 self.fetch_task = None;
//                 // we want to redraw so that the page displays the location of the ISS instead of
//                 // 'fetching...'
//                 true
//             }
//         }
//     }
//     fn view(&self) -> Html {
//         html! {
//             <>
//                 { self.view_fetching() }
//                 { self.view_iss_location() }
//                 { self.view_error() }
//             </>
//         }
//     }
// }








// fn get_lessons() {
//     let resp = Request::get("lessons.js")
//         .send()
//         .await
//         .unwrap();
//     assert_eq!(resp.status(), 200);
//
// }

// #[styled_component(Lesson)]
// pub fn lesson() -> Html {
//
//
//
//
//
//
//
//
//     return html! {
//
//     }
// }
