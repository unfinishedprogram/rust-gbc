#![feature(local_key_cell_methods)]
use gbc_emu::application::{logger::LOGGER, setup_listeners, APPLICATION};
use log::LevelFilter;
use wasm_bindgen::prelude::wasm_bindgen;

#[allow(dead_code)]
#[wasm_bindgen]
pub fn load_rom(rom: &[u8], source: String) {
	let source = serde_json::from_str(&source).unwrap();
	APPLICATION.with_borrow_mut(|app| {
		app.load_rom(rom, Some(source));
		app.start();
	});
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn set_speed(multiplier: f64) {
	APPLICATION.with_borrow_mut(|app| {
		app.set_speed(multiplier);
		app.start();
	});
}

fn main() {
	log::set_logger(&LOGGER).unwrap();
	log::set_max_level(LevelFilter::Info);

	setup_listeners();

	APPLICATION.with_borrow_mut(|app| {
		app.load_rom(
			include_bytes!("../roms/test/blargg/interrupt_time.gb"),
			None,
		);
		app.start();
	});
}
