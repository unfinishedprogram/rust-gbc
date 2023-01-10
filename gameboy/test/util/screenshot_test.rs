use image::EncodableLayout;

use super::rom_loader::init_emulator_with_rom;

pub fn run_screenshot_test(rom: &str, expected: &str, seconds: usize) {
	let mut state = init_emulator_with_rom(rom);

	let expected = image::open(expected).expect("Expected Image does not exist");

	let expected = expected.into_rgba8();
	let expected = expected.as_bytes();

	for _ in 0..seconds {
		for _ in 0..1_048_576 {
			state.step();
		}
		if let Some(lcd) = &state.lcd {
			let actual = lcd.get_current_as_bytes();

			if compare_lcd(actual, expected) {
				return;
			}
		}
	}
	let lcd = state.lcd.expect("No LCD Bound");

	let actual = lcd.get_current_as_bytes();

	if !compare_lcd(actual, expected) {
		panic!("Images are not identical")
	}
}

fn compare_lcd(a: &[u8], b: &[u8]) -> bool {
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

#[macro_export]
macro_rules! screenshot_tests {
    ($($name:ident: ($value:expr, $seconds:expr),)*) => {
		$(
			#[test]
			fn $name() {
				let rom = format!("../roms/test/{}.gb", $value);
				let expected = format!("test/expected/{}.png", $value);
				run_screenshot_test(&rom, &expected, $seconds);
			}
		)*
    }
}
