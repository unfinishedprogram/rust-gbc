use std::fs::read;

use crate::emulator::{lcd::LCD, EmulatorState};
extern crate test;

use lazy_static::lazy_static;
use test::Bencher;

#[bench]
pub fn bench_boot(b: &mut Bencher) {
	let mut state = EmulatorState::default();

	b.iter(|| {
		state.step();
	})
}

lazy_static! {
	pub static ref BOOTED_EMULATOR: EmulatorState = {
		let mut state = EmulatorState::default();
		// Not a specific rom, just one that has a valid logo and will pass checks
		let rom = read("roms/test/dmg-acid2.gb").unwrap();
		let lcd = LCD::new();

		state.bind_lcd(lcd);
		state.load_rom(&rom, "boot_rom".into()).unwrap();

		state.run_until_boot();
		state
	};
}
