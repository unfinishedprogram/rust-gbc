mod app;
mod input;
mod web_save_manager;

fn main() {
	wasm_logger::init(wasm_logger::Config::default());
	console_error_panic_hook::set_once();
	tracing_wasm::set_as_global_default();
	log::set_max_level(log::LevelFilter::Error);
}
