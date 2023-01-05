#![feature(local_key_cell_methods)]

use gbc_emu::application::{setup_listeners, APPLICATION};
use wasm_bindgen::prelude::wasm_bindgen;

#[allow(dead_code)]
#[wasm_bindgen]
pub fn load_rom(rom: &[u8], name: String) {
	APPLICATION.with_borrow_mut(|app| {
		app.load_rom(rom, name);
		app.start();
	});
}

fn main() {
	APPLICATION.with_borrow_mut(|app| {
		app.load_rom(
			include_bytes!("../roms/games/Kirby's Dream Land (USA, Europe).gb"),
			"Kirby".to_string(),
		);
		app.start();
	});
	setup_listeners();
}
