use std::ops::Index;
use std::ops::IndexMut;

use super::values::{as_u16, lsb, msb};

use serde::Serialize;
use CPURegister16::*;
use CPURegister8::*;

#[derive(Copy, Clone, Debug)]
pub enum CPURegister8 {
	A = 0,
	B,
	C,
	D,
	E,
	F,
	H,
	L,
}

#[derive(Copy, Clone, Debug)]
pub enum CPURegister16 {
	AF,
	BC,
	DE,
	HL,
	SP,
	PC,
}
#[derive(Debug, Serialize)]
pub struct CPURegisters {
	pub bytes: [u8; 8],
	pub sp: u16,
	pub pc: u16,
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
			bytes: [0; 8],
			sp: 0,
			pc: 0,
		}
	}

	pub fn get_u8(&self, reg: CPURegister8) -> u8 {
		self.bytes[reg as usize]
	}

	pub fn set_u8(&mut self, reg: CPURegister8, value: u8) {
		self.bytes[reg as usize] = value;
	}

	pub fn get_u16(&self, reg: CPURegister16) -> u16 {
		match reg {
			AF => as_u16([self[F], self[A]]),
			BC => as_u16([self[C], self[B]]),
			DE => as_u16([self[E], self[D]]),
			HL => as_u16([self[L], self[H]]),
			SP => self.sp,
			PC => self.pc,
		}
	}

	pub fn set_u16(&mut self, reg: CPURegister16, value: u16) {
		let bytes = u16::to_le_bytes(value);
		match reg {
			AF => {
				self[A] = bytes[1];
				self[F] = bytes[0];
			}

			BC => {
				self[B] = bytes[1];
				self[C] = bytes[0];
			}

			DE => {
				self[D] = bytes[1];
				self[E] = bytes[0];
			}

			HL => {
				self[H] = bytes[1];
				self[L] = bytes[0];
			}

			SP => self.sp = value,
			PC => self.pc = value,
		}
	}
}
