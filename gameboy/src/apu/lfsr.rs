pub struct Lfsr {
	inner: u16,
}

impl Lfsr {
	pub fn new() -> Self {
		Self { inner: 0x7FFF }
	}

	pub fn step(&mut self, width_mode: bool) -> bool {
		let bit_0 = self.inner & 1 != 0;
		let bit_1 = self.inner & 2 != 0;
		let xor = bit_0 ^ bit_1;
		self.inner >>= 1;

		self.inner |= (xor as u16) << 14;
		if width_mode {
			self.inner |= (xor as u16) << 6;
		}
		!bit_0
	}
}
