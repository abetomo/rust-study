extern crate cfg_if;
extern crate pulldown_cmark;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use pulldown_cmark::{html, Parser};
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    let message: String = format!("Hello, {}", name);
    alert(&message);
    return message;
}

#[wasm_bindgen]
pub fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    return format!("{}", html_buf);
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
