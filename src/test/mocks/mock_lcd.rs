use crate::emulator::lcd::LCDDisplay;

pub struct MockLCD {
	buffer: Vec<(u8, u8, u8)>,
}

impl Default for MockLCD {
	fn default() -> Self {
		Self {
			buffer: vec![(0, 0, 0); 160 * 144],
		}
	}
}

impl LCDDisplay for MockLCD {
	fn get_size(&self) -> (u8, u8) {
		(160, 144)
	}
	fn put_pixel(&mut self, x: u8, y: u8, color: (u8, u8, u8)) {
		let index = y as usize * 160 + x as usize;
		self.buffer[index] = color;
	}
}
