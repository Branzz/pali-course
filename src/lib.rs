#![feature(rustc_private)]
#![feature(rustc_allow_const_fn_unstable)]
#![feature(is_some_and)]
#![feature(fn_traits)]
#![feature(async_closure)]
#![feature(fmt_internals)]

mod contexts;

#[macro_use]
mod app;
use wasm_bindgen::prelude::*;
use yew::Properties;

use yew::Children;

#[no_mangle] // TODO
#[wasm_bindgen(start)]
pub fn lib_main() {
    app::main();
}

fn log_string(s: String) {
    log(s.as_str());
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
    #[wasm_bindgen(js_name = call)]
    fn call();

    #[wasm_bindgen(js_name = prefersDarkScheme)]
    fn prefers_dark_scheme() -> bool;

    #[wasm_bindgen(js_name = sleep)]
    fn sleep(duration_milli: u32);
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct ProviderProps {
    pub children: Children,
}
