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
	B,
	C,
	D,
	E,
	F,
	H,
	L,
}

#[derive(Clone, Copy)]
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
			AF => u16::from_be_bytes([self[A], self[F]]),
			BC => u16::from_be_bytes([self[B], self[C]]),
			DE => u16::from_be_bytes([self[D], self[E]]),
			HL => u16::from_be_bytes([self[H], self[L]]),
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

impl Debug for CPURegisters {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "AF:{:04X}", self.get_u16(AF))?;
		writeln!(f, "BC:{:04X}", self.get_u16(BC))?;
		writeln!(f, "DE:{:04X}", self.get_u16(DE))?;
		writeln!(f, "HL:{:04X}", self.get_u16(HL))?;
		writeln!(f, "SP:{:04X}", self.get_u16(SP))?;
		writeln!(f, "PC:{:04X}", self.get_u16(PC))?;
		Ok(())
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
