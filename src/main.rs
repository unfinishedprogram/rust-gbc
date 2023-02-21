#![feature(local_key_cell_methods)]

use gameboy::{debugger::Breakpoint, Debugger};
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

#[allow(dead_code)]
#[wasm_bindgen]
pub fn set_speed(multiplier: f64) {
	APPLICATION.with_borrow_mut(|app| {
		app.set_speed(multiplier);
		app.start();
	});
}

fn main() {
	wasm_logger::init(wasm_logger::Config::default());
	console_error_panic_hook::set_once();
	tracing_wasm::set_as_global_default();
	setup_listeners();
	Debugger::add_breakpoint(Breakpoint::PPUEnterMode(gameboy::ppu::PPUMode::VBlank));
	APPLICATION.with_borrow_mut(|app| {
		app.load_rom(
			// include_bytes!("../roms/test/mooneye/acceptance/ppu/intr_2_mode3_timing.gb"),
			include_bytes!("../../BullyGB/bully.gb"),
			None,
		);
		app.start();
	});
}
