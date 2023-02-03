use std::path::Path;

use test_generator::test_resources;

use super::util::rom_loader::init_emulator_with_rom;

pub struct GambatteTest {
	pub expected_out: String,
	pub path: String,
}

impl GambatteTest {
	pub fn new(path: String) -> Self {
		let expected_out: Vec<&str> = path.split("_out").collect();
		let expected_out = expected_out
			.last()
			.unwrap()
			.split(".")
			.next()
			.unwrap()
			.to_string();

		GambatteTest { expected_out, path }
	}
}

#[test_resources("../roms/test/gambatte/dma/*.gbc")]
fn verify_resource(resource: &str) {
	let test = GambatteTest::new(resource.to_string());
	get_test_output(&test);

	// assert!(resource.contains("_out"));
}

fn get_test_output(test: &GambatteTest) -> String {
	let mut state = init_emulator_with_rom(&test.path);
	let start_frame = state.ppu.lcd.as_ref().unwrap().frame;

	loop {
		if let Some(lcd) = &state.ppu.lcd {
			if lcd.frame - start_frame > 15 {
				return "DONE".to_string();
			}
		}
		state.step();
	}

	let buffer = state.ppu.lcd.unwrap().get_current_as_bytes();

	"".to_owned()
}

fn screen_as_str(screen: &[u8]) {
	screen.windows(4).map(|[r, g, b, a]| {})
}
