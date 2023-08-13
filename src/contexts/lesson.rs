use gloo_net::Error;
use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use stylist::yew::{Global, styled_component};
use wasm_bindgen_futures::spawn_local;
use yew::{Component, Context, Html, html};
use yew::prelude::*;

use crate::contexts::exercise::{Exercise, ExerciseComponent, ExerciseComponentProps};
use std::ops::Deref;
use crate::{ProviderProps, get_lessons_json};
use serde_wasm_bindgen::from_value;

#[derive(Properties, PartialEq)]
pub struct ExercisesProps {
    pub lesson_path: Option<String>, // for anonymous lessons or single-page exercise
    pub exercises: Vec<Exercise>
}

#[styled_component(Exercises)]
pub fn exercises(props: &ExercisesProps) -> Html {
    return html! {
        { for props.exercises.iter().map(|e| html! {
            <ExerciseComponent lesson_path={props.lesson_path.clone()} exercise={e.clone()} />
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



#[derive(Clone)]
pub(crate) struct LessonsContext {
    inner: UseStateHandle<Lessons>,
}

impl Deref for LessonsContext {
    type Target = Lessons;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl PartialEq for LessonsContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

impl LessonsContext {
    pub fn new(inner: UseStateHandle<Lessons>) -> Self {
        Self { inner }
    }

    pub fn set(&self, lessons: Lessons) {
        self.inner.set(lessons)
    }

    pub fn get_lessons(&self) -> Lessons {
        (*self.inner).clone()
    }

    // pub fn get_lesson(&self, name: String) -> Lesson {
    //
    // }

}

#[hook]
pub(crate) fn use_lessons() -> LessonsContext {
    use_context::<LessonsContext>().unwrap()
}

#[styled_component(LessonsProvider)]
pub(crate) fn theme_provider(props: &ProviderProps) -> Html {
    let lessons_context = LessonsContext::new(use_state_eq(|| from_value::<Lessons>(get_lessons_json()).expect("couldn't load json")));

    html! {
        <ContextProvider<LessonsContext> context={lessons_context}>
            {props.children.clone()}
        </ContextProvider<LessonsContext>>
    }
}
