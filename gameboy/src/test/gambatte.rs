use super::util::rom_loader::init_emulator_with_rom_cgb;
use std::vec;
use test_generator::test_resources;

pub const CHAR_0: u64 = 0b0000000001111111010000010100000101000001010000010100000101111111;
pub const CHAR_1: u64 = 0b0000000000001000000010000000100000001000000010000000100000001000;
pub const CHAR_2: u64 = 0b0000000001111111000000010000000101111111010000000100000001111111;
pub const CHAR_3: u64 = 0b0000000001111111000000010000000100111111000000010000000101111111;
pub const CHAR_4: u64 = 0b0000000001000001010000010100000101111111000000010000000100000001;
pub const CHAR_5: u64 = 0b0000000001111111010000000100000001111110000000010000000101111110;
pub const CHAR_6: u64 = 0b0000000001111111010000000100000001111111010000010100000101111111;
pub const CHAR_7: u64 = 0b0000000001111111000000010000001000000100000010000001000000010000;
pub const CHAR_8: u64 = 0b0000000000111110010000010100000100111110010000010100000100111110;
pub const CHAR_9: u64 = 0b0000000001111111010000010100000101111111000000010000000101111111;
pub const CHAR_A: u64 = 0b0000000000001000001000100100000101111111010000010100000101000001;
pub const CHAR_B: u64 = 0b0000000001111110010000010100000101111110010000010100000101111110;
pub const CHAR_C: u64 = 0b0000000000111110010000010100000001000000010000000100000100111110;
pub const CHAR_D: u64 = 0b0000000001111110010000010100000101000001010000010100000101111110;
pub const CHAR_E: u64 = 0b0000000001111111010000000100000001111111010000000100000001111111;
pub const CHAR_F: u64 = 0b0000000001111111010000000100000001111111010000000100000001000000;

pub fn to_char(val: u64) -> Option<char> {
	match val {
		CHAR_0 => Some('0'),
		CHAR_1 => Some('1'),
		CHAR_2 => Some('2'),
		CHAR_3 => Some('3'),
		CHAR_4 => Some('4'),
		CHAR_5 => Some('5'),
		CHAR_6 => Some('6'),
		CHAR_7 => Some('7'),
		CHAR_8 => Some('8'),
		CHAR_9 => Some('9'),
		CHAR_A => Some('A'),
		CHAR_B => Some('B'),
		CHAR_C => Some('C'),
		CHAR_D => Some('D'),
		CHAR_E => Some('E'),
		CHAR_F => Some('F'),
		0 => Some('_'),
		v => {
			let v = format!("{v:064b}");
			panic!("{v}")
		}
	}
}

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

#[test_resources("../test_data/gambatte/*/*cgb04c_out*.gbc")]
fn gambatte(resource: &str) {
	let test = GambatteTest::new(resource.to_string());
	let output = get_test_output(&test);

	assert_eq!(&output[0..test.expected_out.len()], &test.expected_out);
}

fn get_test_output(test: &GambatteTest) -> String {
	let mut state = init_emulator_with_rom_cgb(&test.path);

	for _ in 0..1053360 / 4 {
		state.step();
	}

	let buffer = state.ppu.lcd.front_buffer();
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
