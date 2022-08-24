
mod instruction;
mod registers;
mod decode_tables;
mod values;

use registers::Registers;

use self::instruction::Opcode;

pub struct CPU<'a> {
	pc:u16,
	sp:u16,
	registers:Registers<'a>,
	memory: [u8; 0xFFFF],
}

impl <'a>CPU {
	fn fetch_next_byte_from_memory(&mut self) -> u8 {
		let value = self.memory[self.pc];
		self.pc += 1;
		return value;
	}

	// fn parse_next_instruction(&mut self) -> (Opcode, Vec<u8>) {
	// 	let mut bytes = Vec::new();
	// 	bytes.push(self.fetch_next_byte_from_memory());

	// 	let has_prefix = match bytes[0] {
	// 		0xCB | 0xDD | 0xED | 0xFD => true,
	// 		_ => false
	// 	};

	// 	if(has_prefix){
	// 		let prefix_byte = self.fetch_next_byte_from_memory();
	// 		bytes.push(prefix_byte);
	// 		let has_second_prefix = match prefix_byte {
	// 			0xDD | 0xFD => bytes[0] == 0xCB,
	// 			_ => false
	// 		};
	// 		if(has_second_prefix) {
	// 			bytes.push(self.fetch_next_byte_from_memory());
	// 			bytes.push(self.fetch_next_byte_from_memory());
	// 		}
	// 	}

	// 	let raw_opcode = match prefix {
	// 		true => self.fetch_next_byte_from_memory(),
	// 		false => prefix_byte
	// 	};

	// 	let opcode = Opcode::new(raw_opcode);
	// 	return (opcode, bytes)
	// }

	pub fn get_register_value(&self, register:Register) {
	}
}
