mod char_mapping;
use std::vec;

use super::util::rom_loader::init_emulator_with_rom;
use char_mapping::to_char;
use test_generator::test_resources;

#[derive(Debug)]
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
			.split('.')
			.next()
			.unwrap()
			.to_string();

		GambatteTest { expected_out, path }
	}
}

#[test_resources("src/test/roms/gambatte/*/*cgb04c_out*.gbc")]
fn exec_test(resource: &str) {
	let test = GambatteTest::new(resource.to_string());
	let output = get_test_output(&test);

	assert_eq!(&output[0..test.expected_out.len()], &test.expected_out);
}

fn get_test_output(test: &GambatteTest) -> String {
	let mut state = init_emulator_with_rom(&test.path);

	for _ in 0..1053360 / 4 {
		state.step();
	}

	let buffer = state.ppu.lcd.as_ref().unwrap().front_buffer();
	screen_as_str(buffer)
}

fn screen_pixels(screen: &[u8]) -> Vec<Vec<bool>> {
	let mut pixels = vec![vec![false; 160]; 144];

	for (y, pixel_row) in pixels.iter_mut().enumerate() {
		for (x, pixel) in pixel_row.iter_mut().enumerate() {
			let index = (x + y * 160) * 4;

			let [r, g, b, a] = [
				screen[index],
				screen[index + 1],
				screen[index + 2],
				screen[index + 3],
			];

			*pixel = match (r, g, b, a) {
				(0, 0, 0, 255) => true,
				(255, 255, 255, 255) => false,
				_ => unreachable!("Must be all black and white"),
			};
		}
	}
	pixels
}

fn pixels_to_tiles(pixels: &[Vec<bool>]) -> Vec<Vec<u64>> {
	let mut tiles = vec![vec![0; 20]; 16];
	for y in 0..16 {
		for x in 0..20 {
			let mut tile = [0; 8];
			for l in 0..8 {
				let mut row: Vec<bool> = pixels[(y * 8) + l][(x * 8)..(x * 8 + 8)].to_vec();
				row.reverse();
				(0..8).for_each(|i| {
					if row[i] {
						tile[l] |= 1 << i
					}
				});
			}
			tiles[y][x] = u64::from_be_bytes(tile);
		}
	}
	tiles
}

fn screen_as_str(screen: &[u8]) -> String {
	let pixels = screen_pixels(screen);
	let tiles = pixels_to_tiles(&pixels);
	tiles
		.iter()
		.flatten()
		.map(|v| to_char(*v).unwrap())
		.collect()
}
