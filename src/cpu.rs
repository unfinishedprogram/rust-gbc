mod registers;
mod values;
mod instruction;

use registers::Registers;

use self::{
	values::{ValueRefU8, ValueRefU16, get_as_u16}, 
};

pub struct CPU {
	pc:u16,
	sp:u16,
	registers:Registers,
	memory: [u8; 0xFFFF],
}

impl <'a>CPU {
	pub fn new() -> CPU {
		CPU {
			pc:0, sp:0, 
			registers:Registers::new(),
			memory: [0;0xFFFF],
		}
	}

	pub fn read_mem(&mut self) -> &mut u8 {
		let value:&mut u8 = &mut self.memory[self.pc as usize];
		self.pc += 1;
		return value;
	}

	pub fn next_byte(&mut self) -> u8 {
		self.pc += 1;
		self.memory[(self.pc-1) as usize]
	}

	pub fn next_chomp(&mut self) -> u16 {
		get_as_u16(&self.next_byte(), &self.next_byte())
	}

	pub fn read_8(&self, value_ref:ValueRefU8) -> u8 {
		match value_ref {
			ValueRefU8::Mem(i) => self.memory[i as usize],
			ValueRefU8::Reg(reg) => self.registers.get_u8(reg),
			ValueRefU8::Raw(x) => x,
		}
	}
	
	pub fn write_8(&mut self, value_ref:ValueRefU8, value:u8) {
		match value_ref {
			ValueRefU8::Mem(i) => self.memory[i as usize] = value,
			ValueRefU8::Reg(reg) => self.registers.set_u8(reg, value),
			ValueRefU8::Raw(_) => unreachable!(),
		}
	}

	pub fn read_16(&self, value_ref:ValueRefU16) -> u16 {
		match value_ref {
			ValueRefU16::Mem(i) => (self.memory[i as usize] as u16) << 8 | self.memory[(i as usize) + 1] as u16,
			ValueRefU16::Reg(reg) => self.registers.get_u16(reg),
			ValueRefU16::Raw(x) => x,
		}
	}

	pub fn write_16(&mut self, value_ref:ValueRefU16, value:u16) {
		match value_ref {
			ValueRefU16::Mem(i) => {
				self.memory[i as usize] = (value >> 8) as u8;
				self.memory[(i as usize) + 1] = (value & 0x00FF) as u8;
			},
			ValueRefU16::Reg(reg) => self.registers.set_u16(reg, value),
			ValueRefU16::Raw(x) => unreachable!(),
		}
	}

	// pub fn get_register_8(&mut self, register: Register8) {

	// }

	// pub fn get_register_16(&mut self, register:Register16) {
	// 	return 
	// }

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
