#![feature(rustc_private)]
#![feature(rustc_allow_const_fn_unstable)]
#![feature(is_some_and)]
#![feature(fn_traits)]
#![feature(async_closure)]
#![feature(fmt_internals)]
#![feature(string_leak)]
#![feature(pattern)]
#![feature(string_remove_matches)]

// #![allow(dead_code)]

use std::fmt::{Debug, Display};
use wasm_bindgen::prelude::*;
use yew::{Html, Properties};
use yew::Children;

use crate::app::empty_html;

mod contexts;

#[macro_use]
mod app;

#[no_mangle] // TODO
#[wasm_bindgen(start)]
pub fn lib_main() {
    app::main();
}

pub fn log_string(s: String) {
    log_str(s.as_str());
}

pub fn log_display<T: Display>(t: T) -> T {
    log_str(format!("{}", t).as_str());
    t
}

pub fn log_dbg<T: Debug>(t: T) -> T {
    log_str(format!("{:?}", t).as_str());
    t
}

pub fn log_str(s: &str) {
    log(s);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_js(v: JsValue);
}

#[wasm_bindgen(module = "/src/main.js")]
extern "C" {
    #[wasm_bindgen(js_name = getTextWidth)]
    fn get_text_width(e: JsValue) -> i32;

    #[wasm_bindgen(js_name = prefersDarkScheme)]
    fn prefers_dark_scheme() -> bool;

    #[wasm_bindgen(js_name = sleep)]
    fn sleep(duration_milli: u32);

    #[wasm_bindgen(js_name = get_lessons_json)]
    fn get_lessons_json() -> JsValue;
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct ProviderProps {
    pub children: Children,
}

pub(crate) fn html_if_some<T, F>(element: Option<T>, html: F) -> Html
    where T: Clone,
          F: Fn(T) -> Html {
    return match element.clone() {
        None => empty_html(),
        Some(val) => html.call((val.clone(),))
    }

}