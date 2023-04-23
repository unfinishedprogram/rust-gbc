use image::{DynamicImage, EncodableLayout};

use test_generator::test_resources;

use crate::test::util::screenshot_test::compare_lcd;

use super::util::rom_loader::{init_emulator_with_rom_cgb, init_emulator_with_rom_dmg};

enum BlarggTest {
	Cgb(String, DynamicImage),
	Dmg(String, DynamicImage),
	DmgCgb(String, DynamicImage, DynamicImage),
	Combined(String, DynamicImage),
}

impl BlarggTest {
	pub fn new(path: &str) -> Self {
		let path = path.to_string();
		let expected_cgb = image::open(path.replace(".gb", "-cgb.png"));
		let expected_dmg = image::open(path.replace(".gb", "-dmg.png"));
		let expected_all = image::open(path.replace(".gb", "-dmg-cgb.png"));

		match (expected_cgb, expected_dmg, expected_all) {
			(_, _, Ok(img)) => Self::Combined(path, img),
			(Ok(dmg), Ok(cgb), _) => Self::DmgCgb(path, dmg, cgb),
			(Ok(dmg), _, _) => Self::Dmg(path, dmg),
			(_, Ok(cgb), _) => Self::Cgb(path, cgb),
			(_, _, _) => panic!("No valid expected image"),
		}
	}

	pub fn run(self) {
		let (mut state, img) = match self {
			BlarggTest::Cgb(path, img) => (init_emulator_with_rom_cgb(&path), img),
			BlarggTest::Dmg(path, img) => (init_emulator_with_rom_dmg(&path), img),
			BlarggTest::DmgCgb(path, _, img) => (init_emulator_with_rom_cgb(&path), img),
			BlarggTest::Combined(path, img) => (init_emulator_with_rom_cgb(&path), img),
		};

		let img = img.into_rgba8();
		let expected = img.as_bytes();

		for _ in 0..30 {
			for _ in 0..1_048_576 {
				state.step();
			}

			let actual = state.ppu.lcd.front_buffer();

			if compare_lcd(actual, expected) {
				return;
			}
		}
		panic!("Images do not match!");
	}
}

#[test_resources("../test_data/blargg/*/*.gb")]
fn blarggs(path: &str) {
	BlarggTest::new(path).run();
}
