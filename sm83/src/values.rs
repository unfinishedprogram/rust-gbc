use std::fmt;

use crate::registers::{CPURegister16, CPURegister8};

impl From<CPURegister16> for ValueRefU16 {
	fn from(value: CPURegister16) -> Self {
		Self::Reg(value)
	}
}

impl From<CPURegister16> for ValueRefU8 {
	fn from(value: CPURegister16) -> Self {
		Self::Mem(value.into())
	}
}

impl From<CPURegister8> for ValueRefU8 {
	fn from(value: CPURegister8) -> Self {
		Self::Reg(value)
	}
}

impl From<u8> for ValueRefU8 {
	fn from(value: u8) -> Self {
		Self::Raw(value)
	}
}

impl From<i8> for ValueRefI8 {
	fn from(value: i8) -> Self {
		Self(value)
	}
}

impl From<u16> for ValueRefU16 {
	fn from(value: u16) -> Self {
		Self::Raw(value)
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ValueRefU8 {
	Reg(CPURegister8),
	Mem(ValueRefU16),
	Raw(u8),
	MemOffsetRaw(u8),
	MemOffsetReg(CPURegister8),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ValueRefU16 {
	Reg(CPURegister16),
	Mem(u16),
	Raw(u16),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ValueRefI8(pub i8);

impl fmt::Debug for ValueRefU16 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use ValueRefU16::*;
		match self {
			Raw(x) => write!(f, "${x:04X}"),
			Mem(x) => write!(f, "[${x:04X}]"),
			Reg(x) => write!(f, "{x:?}"),
		}
	}
}

impl fmt::Debug for ValueRefU8 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use ValueRefU8::*;
		match self {
			Raw(x) => write!(f, "${x:02X}"),
			Mem(x) => write!(f, "[{x:?}]"),
			Reg(x) => write!(f, "{x:?}"),
			MemOffsetRaw(offset) => write!(f, "[${:04X}]", (*offset as u16) + 0xFF00),
			MemOffsetReg(reg) => write!(f, "[{reg:?}]"),
		}
	}
}

impl fmt::Debug for ValueRefI8 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let x = self.0;
		if x >= 0 {
			write!(f, "${x:02X}")
		} else {
			write!(f, "-${:02X}", x.unsigned_abs())
		}
	}
}
