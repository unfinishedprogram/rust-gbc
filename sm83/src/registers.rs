use serde::{Deserialize, Serialize};

use std::fmt::Debug;

pub trait Addressable<Idx, T> {
	fn read(&self, index: Idx) -> T;
	fn write(&mut self, index: Idx, value: T);
}

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

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub struct CPURegisters {
	inner: [u8; 8],
	pc: u16,
	sp: u16,
}

impl Addressable<CPURegister16, u16> for CPURegisters {
	fn read(&self, index: CPURegister16) -> u16 {
		use CPURegister16::*;
		use CPURegister8::*;
		match index {
			AF => u16::from_be_bytes([self.read(A), self.read(F)]),
			BC => u16::from_be_bytes([self.read(B), self.read(C)]),
			DE => u16::from_be_bytes([self.read(D), self.read(E)]),
			HL => u16::from_be_bytes([self.read(H), self.read(L)]),
			SP => self.sp,
			PC => self.pc,
		}
	}

	fn write(&mut self, index: CPURegister16, value: u16) {
		use CPURegister16::*;
		use CPURegister8::*;
		let [high, low] = value.to_be_bytes();
		match index {
			AF => {
				self.write(A, high);
				self.write(F, low);
			}
			BC => {
				self.write(B, high);
				self.write(C, low);
			}
			DE => {
				self.write(D, high);
				self.write(E, low);
			}
			HL => {
				self.write(H, high);
				self.write(L, low);
			}
			SP => self.sp = value,
			PC => self.pc = value,
		}
	}
}

impl Addressable<CPURegister8, u8> for CPURegisters {
	fn read(&self, index: CPURegister8) -> u8 {
		self.inner[index as usize]
	}

	fn write(&mut self, index: CPURegister8, value: u8) {
		match index {
			CPURegister8::F => self.inner[index as usize] = value & 0xF0,
			_ => self.inner[index as usize] = value,
		}
	}
}

impl Debug for CPURegisters {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "AF:{:04X}", self.read(CPURegister16::AF))?;
		writeln!(f, "BC:{:04X}", self.read(CPURegister16::BC))?;
		writeln!(f, "DE:{:04X}", self.read(CPURegister16::DE))?;
		writeln!(f, "HL:{:04X}", self.read(CPURegister16::HL))?;
		writeln!(f, "SP:{:04X}", self.read(CPURegister16::SP))?;
		writeln!(f, "PC:{:04X}", self.read(CPURegister16::PC))
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
