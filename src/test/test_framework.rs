use image::EncodableLayout;
use std::fs::read;

use crate::emulator::{lcd::LCD, EmulatorState};

pub fn run_integration_test(rom: &str, expected: &str, cycles: usize) {
	let mut state = EmulatorState::default();

	let rom = read(rom).expect("Rom does not exist");
	let lcd = LCD::new();

	state.load_rom(&rom).expect("Rom could not be loaded");
	state.bind_lcd(lcd);

	let expected = image::open(expected).expect("Expected Image does not exist");

	// Normalize all images to RGB
	let expected = expected.into_rgb8();
	let expected = expected.as_bytes();

	for _ in 0..cycles {
		for _ in 0..1_048_576 {
			state.step();
		}
		if let Some(lcd) = &state.lcd {
			let actual = lcd.get_current_as_bytes();

			if compare_lcd(&actual, expected) {
				return;
			}
		}
	}

	let actual = &state.lcd.unwrap().get_current_as_bytes();
	if !compare_lcd(actual, expected) {
		panic!("Images are not identical")
	}
}

pub fn compare_lcd(a: &[u8], b: &[u8]) -> bool {
	assert!(
		a.len() == b.len(),
		"Images are not of the same size, a:{}, b:{}",
		a.len(),
		b.len()
	);

	for (a, b) in a.iter().zip(b.iter()) {
		if a != b {
			return false;
		}
	}
	true
}
