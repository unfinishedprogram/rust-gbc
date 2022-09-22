pub struct Lcd {
	pub raw_rgba: [u8; 144 * 160 * 4],
}

impl Lcd {
	pub fn put_pixel(&mut self, x: usize, y: usize, color: (u8, u8, u8, u8)) {
		let index = (y * 160 + x) * 4;
		self.raw_rgba[index + 0] = color.0;
		self.raw_rgba[index + 1] = color.1;
		self.raw_rgba[index + 2] = color.2;
		self.raw_rgba[index + 3] = color.3;
	}
}
