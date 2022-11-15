pub trait LCDDisplay {
	fn get_size(&self) -> (u8, u8);
	fn put_pixel(&mut self, x: u8, y: u8, color: (u8, u8, u8));
	fn get_image_data(&self) -> &Vec<u8>;
}

pub struct LCD {
	buffer: Vec<u8>,
}

impl LCDDisplay for LCD {
	fn get_size(&self) -> (u8, u8) {
		(160, 144)
	}

	fn put_pixel(&mut self, x: u8, y: u8, color: (u8, u8, u8)) {
		let (width, _) = self.get_size();
		let (r, g, b) = color;
		let index: usize = (y as usize * width as usize + x as usize) * 4;

		self.buffer[index + 0] = r;
		self.buffer[index + 1] = g;
		self.buffer[index + 2] = b;
	}

	fn get_image_data(&self) -> &Vec<u8> {
		&self.buffer
	}
}

impl LCD {
	pub fn new() -> Self {
		Self {
			buffer: vec![0xFF; 144 * 160 * 4],
		}
	}
}
