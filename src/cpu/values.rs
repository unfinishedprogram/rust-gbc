use super::registers::Register8;
use super::registers::Register16;

pub enum ValueRefU8 {
	Reg(Register8),
	Mem(u16),
	Raw(u8)
}

pub enum ValueRefU16 {
	Reg(Register16),
	Mem(u16),
	Raw(u16)
}

pub enum ValueRefI8 {
	Reg(Register8),
	Mem(u8),
	Raw(i8)
}

pub enum ValueRefI16 {
	Reg(Register16),
	Mem(u16),
	Raw(i16)
}

pub fn get_as_u16(big:&u8, small:&u8) -> u16 {
	(*big as u16) << 8 | *small as u16
}

pub fn set_as_u16(big:&mut u8, small:&mut u8, value:u16) {
	*big  = ((value & 0xFF00) >> 8) as u8;
	*small = (value & 0xFF) as u8;
}

pub fn set_as_u16_big(byte:&mut u8, value:u16) {
	*byte  = ((value & 0xFF00) >> 8) as u8;
}
pub fn set_as_u16_small(byte:&mut u8, value:u16) {
	*byte = (value & 0xFF) as u8;
}