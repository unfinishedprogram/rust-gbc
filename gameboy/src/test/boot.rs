use std::fs::read;

use crate::{lcd::LCD, Gameboy};
extern crate test;

use lazy_static::lazy_static;
use test::Bencher;

#[bench]
pub fn bench_boot(b: &mut Bencher) {
	let mut state = Gameboy::default();

	b.iter(|| {
		state.step();
	})
}

lazy_static! {
	pub static ref BOOTED_EMULATOR: Gameboy = {
		let mut state = Gameboy::default();
		// Not a specific rom, just one that has a valid logo and will pass checks
		// TODO: Make this a custom rom that minimally satisfies the boot requirements
		let rom = read("../roms/test/dmg-acid2.gb").unwrap();
		let lcd = LCD::new();

		state.bind_lcd(lcd);
		state.load_rom(&rom, None).unwrap();

		state.run_until_boot();
		state
	};
}
