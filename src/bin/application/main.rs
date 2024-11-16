mod app;
mod audio;
mod callback;
mod input;
mod screen;
mod uploader;
mod web_save_manager;

pub use app::APPLICATION;

use uploader::setup_upload_listeners;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn load_rom(rom: &[u8], source: String) {
	let source = serde_json::from_str(&source).unwrap();
	APPLICATION.with_borrow_mut(|app| {
		app.load_rom(rom, Some(source));
		app.start();
	});
}

#[wasm_bindgen]
pub fn set_speed(multiplier: f64) {
	APPLICATION.with_borrow_mut(|app| {
		app.set_speed(multiplier);
		app.start();
	});
}

#[wasm_bindgen]
pub fn set_vsync(v_sync: bool) {
	APPLICATION.with_borrow_mut(|app| {
		app.set_v_sync(v_sync);
	});
}

#[wasm_bindgen]
pub fn step_emulator() {
	APPLICATION.with_borrow_mut(|app| {
		app.step_single();
	})
}

#[wasm_bindgen]
pub fn toggle_play() -> bool {
	APPLICATION.with_borrow_mut(|app| app.toggle_play())
}

fn main() {
	wasm_logger::init(wasm_logger::Config::default());
	console_error_panic_hook::set_once();
	tracing_wasm::set_as_global_default();

	log::set_max_level(log::LevelFilter::Error);
	setup_upload_listeners();

	APPLICATION.with_borrow_mut(|app| app.start());
}
