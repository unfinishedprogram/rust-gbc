struct Registers {
	data:[u8;8] // index 5 is flag register
}

impl Registers {
	fn get_u8(&self, index:u8) -> u8 {
		self.data[index]
	}

	fn set_u8(&mut self, index:u8, value:u8) {
		self.data[index] = value;
	}

	fn get_u16(&self, index:u8) {
		(self.data[index] as u16) << 8 | self.data[index + 1] as u16
	}

	fn set_u16(&mut self, index:u8, value:u8) {
		self.data[index] = ((value & 0xFF00) >> 8) as u8;
    self.data[index+1] = (value & 0xFF) as u8;
	}
}