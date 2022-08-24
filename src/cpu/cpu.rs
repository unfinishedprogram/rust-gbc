pub struct CPU {
	pc:u16,
	registers:Registers,
	memory: [u8; 0xFFFF],
}

impl CPU {
	fn execute(instruction:Instruction) {

	}

	fn fetch_next_byte(&mut self) {
		let value = self.memory[self.pc];
		self.pc += 1;
		return value;
	}

	fn decode_next_instruction(&mut self) {
		let mut bytes = Vec::new();
		vec.push(self.fetch_next_byte());

		let has_prefix = match vec[0] {
			0xCB | 0xDD | 0xED | 0xFD => true,
			_ => false
		};

		if(has_prefix) {
			bytes.push(self.fetch_next_byte());
		}

		let has_second_prefix = match prefix_byte {
			0xDD | 0xFD => bytes[0] == 0xCB,
			_ => false
		};

		if(has_second_prefix) {
			bytes.push(self.fetch_next_byte());
			bytes.push(self.fetch_next_byte());
		}

		opcode = match prefix {
			true => self.fetch_next_byte(),
			false => prefix_byte
		};
	}
}