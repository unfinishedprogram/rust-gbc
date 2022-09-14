use std::ops::Index;
use std::ops::IndexMut;

use super::values::get_as_u16;
use super::values::set_as_u16_big;
use super::values::set_as_u16_small;

use CPURegister16::*;
use CPURegister8::*;
use serde::Serialize;

#[derive(Copy, Clone, Debug)]
pub enum CPURegister8 { A=0, B, C, D, E, F, H, L }

#[derive(Copy, Clone, Debug)]
pub enum CPURegister16 {
	AF, BC,
	DE, HL, 
	SP, PC, 
}
#[derive(Debug, Serialize)]
pub struct CPURegisters {
	pub bytes:[u8;8],
	pub sp:u16,
	pub pc:u16,
}

impl Index<CPURegister8> for CPURegisters {
	type Output = u8;
	fn index(&self, reg: CPURegister8) -> &Self::Output {
		&self.bytes[reg as usize]
	}
}

impl IndexMut<CPURegister8> for CPURegisters {
	fn index_mut(&mut self, reg: CPURegister8) -> &mut Self::Output {
		&mut self.bytes[reg as usize]
	}
}

impl CPURegisters {
	pub fn new() -> CPURegisters {
		CPURegisters {
			bytes:[0; 8],
			sp:0, 
			pc:0,
		}
	}

	pub fn get_u8(&self, reg:CPURegister8) -> u8 {
		self.bytes[reg as usize]
	}

	pub fn set_u8(&mut self, reg:CPURegister8, value:u8) {
		self.bytes[reg as usize] = value;
	}

	pub fn get_u16(&self, reg:CPURegister16) -> u16 {
		match reg {
			AF => get_as_u16(&self[A], &self[F]),
			BC => get_as_u16(&self[B], &self[C]),
			DE => get_as_u16(&self[D], &self[E]),
			HL => get_as_u16(&self[H], &self[L]),
			SP => self.sp,
			PC => self.pc,
		}
	}

	pub fn set_u16(&mut self, reg:CPURegister16, value:u16) {
		match reg {
			AF => {
				set_as_u16_big(&mut self[A], value);
				set_as_u16_small(&mut self[F], value);
			},

			BC => {
				set_as_u16_big(&mut self[B], value);
				set_as_u16_small(&mut self[C], value);
			},

			DE => {
				set_as_u16_big(&mut self[D], value);
				set_as_u16_small(&mut self[E], value);
			},
			
			HL => {
				set_as_u16_big(&mut self[H], value);
				set_as_u16_small(&mut self[L], value);
			},

			SP => self.sp = value,
			PC => self.pc = value,
		}
	}
}