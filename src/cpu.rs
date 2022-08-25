
mod instruction;
mod registers;
mod decode_tables;
mod values;

use registers::Registers;

use self::{instruction::Opcode, registers::CompositeU16};

pub struct CPU<'a> {
	pc:u16,
	sp:u16,
	registers:Registers<'a>,
	memory: [u8; 0xFFFF],
}

impl <'a>CPU<'_> {
	pub fn read_mem(&mut self) -> &mut u8 {
		let value:&mut u8 = &self.memory[self.pc];
		self.pc += 1;
		return value;
	}

	pub fn read_nn(&mut self) -> &mut CompositeU16 {
		return &mut CompositeU16(self.read_mem(), self.read_mem());
	}

	// fn parse_next_instruction(&mut self) -> (Opcode, Vec<u8>) {
	// 	let mut bytes = Vec::new();
	// 	bytes.push(self.read_mem());

	// 	let has_prefix = match bytes[0] {
	// 		0xCB | 0xDD | 0xED | 0xFD => true,
	// 		_ => false
	// 	};

	// 	if(has_prefix){
	// 		let prefix_byte = self.read_mem();
	// 		bytes.push(prefix_byte);
	// 		let has_second_prefix = match prefix_byte {
	// 			0xDD | 0xFD => bytes[0] == 0xCB,
	// 			_ => false
	// 		};
	// 		if(has_second_prefix) {
	// 			bytes.push(self.read_mem());
	// 			bytes.push(self.read_mem());
	// 		}
	// 	}

	// 	let raw_opcode = match prefix {
	// 		true => self.read_mem(),
	// 		false => prefix_byte
	// 	};

	// 	let opcode = Opcode::new(raw_opcode);
	// 	return (opcode, bytes)
	// }
}
