use serde::{Deserialize, Serialize};

use std::{
	fmt::Debug,
	ops::{Index, IndexMut},
};

const FLAG_MASK: u128 = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF;

#[derive(Clone, Copy)]
pub enum CPURegister8 {
	F,
	A,
	C,
	B,
	E,
	D,
	L,
	H,
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

#[derive(Clone, Copy)]
#[repr(C)]
pub union CPURegisters {
	pub bits_8: [u8; 12],
	pub bits_16: [u16; 6],
	pub bits_128: u128,
}

impl Index<CPURegister8> for CPURegisters {
	type Output = u8;
	fn index(&self, reg: CPURegister8) -> &Self::Output {
		unsafe { &self.bits_8[reg as usize] }
	}
}

impl IndexMut<CPURegister8> for CPURegisters {
	fn index_mut(&mut self, reg: CPURegister8) -> &mut Self::Output {
		unsafe { &mut self.bits_8[reg as usize] }
	}
}

impl Index<CPURegister16> for CPURegisters {
	type Output = u16;
	fn index(&self, reg: CPURegister16) -> &Self::Output {
		unsafe { &self.bits_16[reg as usize] }
	}
}

impl IndexMut<CPURegister16> for CPURegisters {
	fn index_mut(&mut self, reg: CPURegister16) -> &mut Self::Output {
		unsafe { &mut self.bits_16[reg as usize] }
	}
}

impl Serialize for CPURegisters {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		todo!()
	}
}

impl<'de> Deserialize<'de> for CPURegisters {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		todo!()
	}
}

impl Default for CPURegisters {
	fn default() -> Self {
		Self { bits_128: 0 }
	}
}

impl Debug for CPURegisters {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "AF:{:04X}", self[CPURegister16::AF])?;
		writeln!(f, "BC:{:04X}", self[CPURegister16::BC])?;
		writeln!(f, "DE:{:04X}", self[CPURegister16::DE])?;
		writeln!(f, "HL:{:04X}", self[CPURegister16::HL])?;
		writeln!(f, "SP:{:04X}", self[CPURegister16::SP])?;
		writeln!(f, "PC:{:04X}", self[CPURegister16::PC])
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
