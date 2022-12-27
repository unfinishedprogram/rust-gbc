use std::fmt::Debug;
use std::ops::Index;
use std::ops::IndexMut;

use CPURegister16::*;
use CPURegister8::*;

#[derive(Clone)]
pub enum CPURegister8 {
	A,
	B,
	C,
	D,
	E,
	F,
	H,
	L,
}

#[derive(Clone)]
pub enum CPURegister16 {
	AF,
	BC,
	DE,
	HL,
	SP,
	PC,
}

#[derive(Debug, Clone)]
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

impl Default for CPURegisters {
	fn default() -> Self {
		Self {
			bytes: [0; 8],
			sp: 0,
			pc: 0x0100,
		}
	}
}

impl CPURegisters {
	pub fn get_u16(&self, reg: CPURegister16) -> u16 {
		match reg {
			AF => u16::from_le_bytes([self[F], self[A]]),
			BC => u16::from_le_bytes([self[C], self[B]]),
			DE => u16::from_le_bytes([self[E], self[D]]),
			HL => u16::from_le_bytes([self[L], self[H]]),
			SP => self.sp,
			PC => self.pc,
		}
	}

	pub fn set_u16(&mut self, reg: CPURegister16, value: u16) {
		let bytes = u16::to_le_bytes(value);

		match reg {
			AF => [self[F], self[A]] = [bytes[0] & 0xF0, bytes[1]],
			BC => [self[C], self[B]] = bytes,
			DE => [self[E], self[D]] = bytes,
			HL => [self[L], self[H]] = bytes,

			SP => self.sp = value,
			PC => self.pc = value,
		}
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
