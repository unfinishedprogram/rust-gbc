use crate::Gameboy;
use lazy_static::lazy_static;

lazy_static! {
	static ref BOOTED_DMG: Gameboy = {
		let mut state = Gameboy::dmg();
		// Not a specific rom, just one that has a valid logo and will pass checks
		let rom = *include_bytes!("../../../test_data/dmg-acid2/dmg-acid2.gb");

		state.load_rom(&rom, None);

		state.run_until_boot();
		state
	};

	static ref BOOTED_CGB: Gameboy = {
		let mut state = Gameboy::cgb();
		// Not a specific rom, just one that has a valid logo and will pass checks
		let rom = *include_bytes!("../../../test_data/cgb-acid2/cgb-acid2.gbc");

		state.load_rom(&rom, None);

		state.run_until_boot();
		state
	};
}

pub fn cgb_test_instance() -> Gameboy {
	BOOTED_CGB.clone()
}

pub fn dmg_test_instance() -> Gameboy {
	BOOTED_DMG.clone()
}
