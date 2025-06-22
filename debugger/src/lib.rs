#![feature(iter_map_windows)]

pub mod components;
mod debugger;
mod memory_map;
pub use debugger::Debugger;

pub fn run_debugger() {
	wasm_logger::init(wasm_logger::Config::default());
	console_error_panic_hook::set_once();
	tracing_wasm::set_as_global_default();

	#[cfg(target_arch = "wasm32")]
	wasm_bindgen_futures::spawn_local(async {
		let runner = eframe::WebRunner::new();
		runner
			.start(
				"canvas",
				eframe::WebOptions::default(),
				Box::new(|cc| Ok(Box::new(Debugger::new(cc)))),
			)
			.await
			.unwrap();
	});
}
