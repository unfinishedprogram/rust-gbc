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
	// APPLICATION.with_borrow_mut(|app| {});
	setup_listeners();
}
