use std::ops::Index;
use std::ops::IndexMut;

use super::values::get_as_u16;
use super::values::set_as_u16_big;
use super::values::set_as_u16_small;

#[derive(Copy, Clone)]
pub enum Register8 { A=0, B, C, D, E, F, H, L }

#[derive(Copy, Clone)]
pub enum Register16 {
	AF, BC,
	DE, HL, 
	SP, PC, 
}

pub struct Registers {
	bytes:[u8;8],
	pub sp:u16,
	pub pc:u16,
}

impl Index<Register8> for Registers {
	type Output = u8;
	fn index(&self, reg: Register8) -> &Self::Output {
		&self.bytes[reg as usize]
	}
}

impl IndexMut<Register8> for Registers {
	fn index_mut(&mut self, reg: Register8) -> &mut Self::Output {
		&mut self.bytes[reg as usize]
	}
}

impl Registers {
	pub fn new() -> Registers {
		Registers {
			bytes:[0; 8],
			sp:0, 
			pc:0,
		}
	}

	pub fn get_u8(&self, reg:Register8) -> u8 {
		self.bytes[reg as usize]
	}

	pub fn set_u8(&mut self, reg:Register8, value:u8) {
		self.bytes[reg as usize] = value;
	}

	pub fn get_u16(&self, reg:Register16) -> u16 {
		match reg {
			Register16::AF => get_as_u16(&self[Register8::A], &self[Register8::F]),
			Register16::BC => get_as_u16(&self[Register8::B], &self[Register8::C]),
			Register16::DE => get_as_u16(&self[Register8::D], &self[Register8::E]),
			Register16::HL => get_as_u16(&self[Register8::H], &self[Register8::L]),
			Register16::SP => self.sp,
			Register16::PC => self.pc,
		}
	}

	pub fn set_u16(&mut self, reg:Register16, value:u16) {
		match reg {
			Register16::AF => {
				set_as_u16_big(&mut self[Register8::A], value);
				set_as_u16_small(&mut self[Register8::F], value);
			},

			Register16::BC => {
				set_as_u16_big(&mut self[Register8::B], value);
				set_as_u16_small(&mut self[Register8::C], value);
			},

			Register16::DE => {
				set_as_u16_big(&mut self[Register8::D], value);
				set_as_u16_small(&mut self[Register8::E], value);
			},
			
			Register16::HL => {
				set_as_u16_big(&mut self[Register8::H], value);
				set_as_u16_small(&mut self[Register8::L], value);
			},

			Register16::SP => self.sp = value,
			Register16::PC => self.pc = value,
		}
	}
}