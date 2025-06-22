use std::fs::read;

use crate::{
	cartridge::Cartridge,
	test::boot::{cgb_test_instance, dmg_test_instance},
	Gameboy,
};

// We want to cache the boot-rom execution, however it will change based on CGB compatibility of the rom
// Therefore, we check the compatibility info in the header
pub fn init_emulator_with_rom(src: &str) -> Gameboy {
	let rom = read(src).unwrap();
	let cart = Cartridge::try_new(&rom, None).unwrap();

	let mut state = if cart.info.cgb {
		cgb_test_instance()
	} else {
		dmg_test_instance()
	};

	state.load_rom(&rom, None);
	state
}

pub fn init_emulator_with_rom_cgb(src: &str) -> Gameboy {
	init_emulator_with_rom(src)
}

pub fn init_emulator_with_rom_dmg(src: &str) -> Gameboy {
	init_emulator_with_rom(src)
}
