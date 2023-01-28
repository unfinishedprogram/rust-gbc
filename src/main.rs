#![feature(local_key_cell_methods)]

use gbc_emu::application::{setup_listeners, APPLICATION};
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

fn main() {
	console_error_panic_hook::set_once();
	tracing_wasm::set_as_global_default();
	setup_listeners();
	// APPLICATION.with_borrow_mut(|app| {
	// 	app.load_rom(include_bytes!("../roms/games/dmg-acid2.gb"), None);
	// 	app.start();
	// });
}
