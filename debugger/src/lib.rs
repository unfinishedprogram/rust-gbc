#![feature(iter_map_windows)]

pub mod components;
mod debugger;
mod memory_map;
pub use debugger::Debugger;

#[cfg(target_arch = "wasm32")]
use eframe::{wasm_bindgen::JsCast, web_sys};

pub fn run_debugger() {
	wasm_logger::init(wasm_logger::Config::default());
	console_error_panic_hook::set_once();
	tracing_wasm::set_as_global_default();

	#[cfg(target_arch = "wasm32")]
	let canvas = web_sys::window()
		.and_then(|win| win.document())
		.and_then(|doc| doc.get_element_by_id("canvas"))
		.and_then(|el| el.dyn_into::<web_sys::HtmlCanvasElement>().ok())
		.expect("Failed to find canvas element");

	#[cfg(target_arch = "wasm32")]
	wasm_bindgen_futures::spawn_local(async {
		let runner = eframe::WebRunner::new();

		runner
			.start(
				canvas,
				eframe::WebOptions::default(),
				Box::new(|cc| Ok(Box::new(Debugger::new(cc)))),
			)
			.await
			.unwrap();
	});
}
