use serde::{Deserialize, Serialize};

use std::{
	fmt::Debug,
	ops::{Index, IndexMut},
};

use CPURegister16::*;
use CPURegister8::*;

#[derive(Clone, Copy)]
pub enum CPURegister8 {
	A,
	F,
	B,
	C,
	D,
	E,
	H,
	L,
}

#[derive(Clone, Copy)]
#[repr(C, align(2))]
pub enum CPURegister16 {
	AF,
	BC,
	DE,
	HL,
	SP,
	PC,
}

#[derive(Clone, Serialize, Deserialize, Default)]
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
	pub fn get_u16(&self, reg: CPURegister16) -> u16 {
		match reg {
			SP => self.sp,
			PC => self.pc,
			reg => u16::from_le_bytes([
				self.bytes[reg as usize * 2 + 1],
				self.bytes[reg as usize * 2],
			]),
		}
	}

	pub fn set_u16(&mut self, reg: CPURegister16, value: u16) {
		let bytes = u16::to_le_bytes(value);

		match reg {
			AF => [self.bytes[F as usize], self.bytes[A as usize]] = [bytes[0] & 0xF0, bytes[1]],
			SP => self.sp = value,
			PC => self.pc = value,
			reg => {
				[
					self.bytes[reg as usize * 2 + 1],
					self.bytes[reg as usize * 2],
				] = bytes
			}
		}
	}
}

impl Debug for CPURegisters {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "AF:{:04X}", self.get_u16(AF))?;
		writeln!(f, "BC:{:04X}", self.get_u16(BC))?;
		writeln!(f, "DE:{:04X}", self.get_u16(DE))?;
		writeln!(f, "HL:{:04X}", self.get_u16(HL))?;
		writeln!(f, "SP:{:04X}", self.get_u16(SP))?;
		writeln!(f, "PC:{:04X}", self.get_u16(PC))
	}
}

impl Debug for CPURegister8 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::A => write!(f, "a"),
			Self::B => write!(f, "b"),
			Self::C => write!(f, "c"),
			Self::D => write!(f, "d"),
			Self::E => write!(f, "e"),
			Self::F => write!(f, "f"),
			Self::H => write!(f, "h"),
			Self::L => write!(f, "l"),
		}
	}
}

impl Debug for CPURegister16 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::AF => write!(f, "af"),
			Self::BC => write!(f, "bc"),
			Self::DE => write!(f, "de"),
			Self::HL => write!(f, "hl"),
			Self::SP => write!(f, "sp"),
			Self::PC => write!(f, "pc"),
		}
	}
}
