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

pub fn get_as_u16(small: u8, big: u8) -> u16 {
	(big as u16) << 8 | small as u16
}

// pub fn set_as_u16(big: &mut u8, small: &mut u8, value: u16) {
// 	*big = ((value & 0xFF00) >> 8) as u8;
// 	*small = (value & 0xFF) as u8;
// }

pub fn set_as_u16_big(byte: &mut u8, value: u16) {
	*byte = ((value & 0xFF00) >> 8) as u8;
}

pub fn set_as_u16_small(byte: &mut u8, value: u16) {
	*byte = (value & 0xFF) as u8;
}
