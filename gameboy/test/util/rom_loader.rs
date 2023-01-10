use std::fs::read;

use crate::{test::boot::BOOTED_EMULATOR, Gameboy};

pub fn init_emulator_with_rom(src: &str) -> Gameboy {
	let mut state = BOOTED_EMULATOR.clone();

	let rom = read(src).unwrap();

	state.load_rom(&rom, None).expect("Rom could not be loaded");

	state
}
