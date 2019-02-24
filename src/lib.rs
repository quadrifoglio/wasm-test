use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(module = "../js/callback")]
extern "C" {
    fn callback(message: String);
}

#[wasm_bindgen]
pub fn change_color(r: u8, g: u8, b: u8) {
    let document = window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let color = format!("rgb({}, {}, {})", r, g, b);

    ctx.set_fill_style(&JsValue::from_str(&color));
    ctx.fill_rect(0.0, 0.0, 800.0, 600.0);

    callback(color);
}
