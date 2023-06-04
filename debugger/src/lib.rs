#![feature(exclusive_range_pattern)]
#![feature(slice_as_chunks)]

pub mod components;
mod debugger;
mod memory_map;
pub use debugger::Debugger;

pub fn run_debugger() {
	console_error_panic_hook::set_once();
	tracing_wasm::set_as_global_default();

	#[cfg(target_arch = "wasm32")]
	let web_options = eframe::WebOptions::default();

	wasm_bindgen_futures::spawn_local(async {
		#[cfg(target_arch = "wasm32")]
		eframe::start_web(
			"canvas",
			web_options,
			Box::new(|cc| Box::new(Debugger::new(cc))),
		)
		.await
		.expect("failed to start eframe");
	});
}
