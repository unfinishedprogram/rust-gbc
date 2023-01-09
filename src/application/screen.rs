use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

pub fn get_screen_ctx() -> CanvasRenderingContext2d {
	document()
		.get_element_by_id("screen")
		.unwrap()
		.dyn_into::<web_sys::HtmlCanvasElement>()
		.unwrap()
		.get_context("2d")
		.unwrap()
		.unwrap()
		.dyn_into::<web_sys::CanvasRenderingContext2d>()
		.unwrap()
}
