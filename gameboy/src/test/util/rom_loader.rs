use std::fs::read;

use crate::{test::boot::BOOTED_EMULATOR, Gameboy};

pub fn init_emulator_with_rom(src: &str) -> Gameboy {
	let mut state = BOOTED_EMULATOR.clone();

	let rom = read(src).expect(src);

	state.load_rom(&rom, None);

	state
}
