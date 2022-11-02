use super::registers::CPURegister16;
use super::registers::CPURegister8;
use std::fmt;

impl Into<ValueRefU16> for CPURegister16 {
	fn into(self: CPURegister16) -> ValueRefU16 {
		ValueRefU16::Reg(self)
	}
}

impl Into<ValueRefU8> for CPURegister16 {
	fn into(self: CPURegister16) -> ValueRefU8 {
		ValueRefU8::Mem(self.into())
	}
}

impl Into<ValueRefU8> for CPURegister8 {
	fn into(self: CPURegister8) -> ValueRefU8 {
		ValueRefU8::Reg(self)
	}
}

impl Into<ValueRefU8> for u8 {
	fn into(self: u8) -> ValueRefU8 {
		ValueRefU8::Raw(self)
	}
}

impl Into<ValueRefI8> for i8 {
	fn into(self: i8) -> ValueRefI8 {
		ValueRefI8::Raw(self)
	}
}

impl Into<ValueRefU16> for u16 {
	fn into(self: u16) -> ValueRefU16 {
		ValueRefU16::Raw(self)
	}
}

#[derive(Copy, Clone)]
pub enum ValueRefU8 {
	Reg(CPURegister8),
	Mem(ValueRefU16),
	Raw(u8),
}

#[derive(Copy, Clone)]
pub enum ValueRefU16 {
	Reg(CPURegister16),
	Mem(u16),
	Raw(u16),
}

#[derive(Copy, Clone)]
pub enum ValueRefI8 {
	Reg(CPURegister8),
	Mem(u16),
	Raw(i8),
}

impl fmt::Debug for ValueRefU16 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ValueRefU16::Raw(x) => write!(f, "${:04X}", x),
			ValueRefU16::Mem(x) => write!(f, "(${:04X})", x),
			ValueRefU16::Reg(x) => write!(f, "{:?}", x),
		}
	}
}

impl fmt::Debug for ValueRefU8 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ValueRefU8::Raw(x) => write!(f, "${:02X}", x),
			ValueRefU8::Mem(x) => write!(f, "[{:?}]", x),
			ValueRefU8::Reg(x) => write!(f, "{:?}", x),
		}
	}
}

impl fmt::Debug for ValueRefI8 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ValueRefI8::Raw(x) => write!(f, "${:02X}", x),
			ValueRefI8::Mem(x) => write!(f, "[{:?}]", x),
			ValueRefI8::Reg(x) => write!(f, "{:?}", x),
		}
	}
}
