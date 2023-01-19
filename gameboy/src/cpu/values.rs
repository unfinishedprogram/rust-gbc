use super::registers::CPURegister16;
use super::registers::CPURegister8;
use std::fmt;

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
		Self::Raw(value)
	}
}
impl From<u16> for ValueRefU16 {
	fn from(value: u16) -> Self {
		Self::Raw(value)
	}
}

#[derive(Clone, Copy)]
pub enum ValueRefU8 {
	Reg(CPURegister8),
	Mem(ValueRefU16),
	MemOffsetRaw(u8),
	MemOffsetReg(CPURegister8),
	Raw(u8),
}

#[derive(Clone, Copy)]
pub enum ValueRefU16 {
	Reg(CPURegister16),
	Mem(u16),
	Raw(u16),
}

#[derive(Clone, Copy)]
pub enum ValueRefI8 {
	Reg(CPURegister8),
	Mem(u16),
	Raw(i8),
}

impl fmt::Debug for ValueRefU16 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ValueRefU16::Raw(x) => write!(f, "${x:04X}"),
			ValueRefU16::Mem(x) => write!(f, "[${x:04X}]"),
			ValueRefU16::Reg(x) => write!(f, "{x:?}"),
		}
	}
}

impl fmt::Debug for ValueRefU8 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ValueRefU8::Raw(x) => write!(f, "${x:02X}"),
			ValueRefU8::Mem(x) => write!(f, "[{x:?}]"),
			ValueRefU8::Reg(x) => write!(f, "{x:?}"),
			ValueRefU8::MemOffsetRaw(offset) => write!(f, "[${:04X}]", (*offset as u16) + 0xFF00),
			ValueRefU8::MemOffsetReg(reg) => write!(f, "[{reg:?}]"),
		}
	}
}

impl fmt::Debug for ValueRefI8 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ValueRefI8::Raw(x) => {
				if *x >= 0 {
					write!(f, "${x:02X}")
				} else {
					write!(f, "-${:02X}", x.unsigned_abs())
				}
			}
			ValueRefI8::Mem(x) => write!(f, "[{x:?}]"),
			ValueRefI8::Reg(x) => write!(f, "{x:?}"),
		}
	}
}
