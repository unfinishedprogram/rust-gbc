pub struct Memory {
	bytes: [u8; 0x10000],
}

impl Memory {
	pub fn new() -> Self {
		Self {
			bytes: [0; 0x10000],
		}
	}

	pub fn read(&self, addr: u16) -> u8 {
		return match addr {
			0xFEA0..=0xFEFF => 0,
			_ => self.bytes[addr as usize],
		};
	}

	pub fn get_ref(&mut self, addr: u16) -> &mut u8 {
		return &mut self.bytes[addr as usize];
	}

	pub fn write(&mut self, addr: u16, value: u8) {
		match addr {
			0xFEA0..=0xFEFF => {}
			0xFF26 => {
				self.bytes[addr as usize] =
					(self.bytes[addr as usize] & 0b01110000) | (value & 0b10000000)
			}
			_ => self.bytes[addr as usize] = value,
		}
	}
}
