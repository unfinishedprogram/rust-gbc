use super::registers::CPURegister16;
use super::registers::CPURegister8;

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

#[derive(Copy, Clone, Debug)]
pub enum ValueRefU8 {
	Reg(CPURegister8),
	Mem(ValueRefU16),
	Raw(u8),
}

#[derive(Copy, Clone, Debug)]
pub enum ValueRefU16 {
	Reg(CPURegister16),
	Mem(u16),
	Raw(u16),
}

#[derive(Copy, Clone, Debug)]
pub enum ValueRefI8 {
	Reg(CPURegister8),
	Mem(u16),
	Raw(i8),
}

pub fn as_u16(bytes: [u8; 2]) -> u16 {
	u16::from_le_bytes(bytes)
}
