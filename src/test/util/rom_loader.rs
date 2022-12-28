use std::fs::read;

use crate::emulator::{lcd::LCD, EmulatorState};

pub fn init_emulator_with_rom(rom: &str) -> EmulatorState {
	let mut state = EmulatorState::default();
	let rom = read(rom).unwrap();
	let lcd = LCD::new();

	state.load_rom(&rom).expect("Rom could not be loaded");
	state.bind_lcd(lcd);

	state
}
