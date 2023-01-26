use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub fn get_screen_ctx() -> CanvasRenderingContext2d {
	document()
		.get_element_by_id("screen")
		.unwrap()
		.dyn_into::<HtmlCanvasElement>()
		.unwrap()
		.get_context("2d")
		.unwrap()
		.unwrap()
		.dyn_into::<CanvasRenderingContext2d>()
		.unwrap()
}
