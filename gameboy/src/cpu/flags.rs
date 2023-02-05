use crate::util::bits::*;

pub const Z: u8 = BIT_7;
pub const N: u8 = BIT_6;
pub const H: u8 = BIT_5;
pub const C: u8 = BIT_4;

pub trait Flags {
	fn get_flag_byte_mut(&mut self) -> &mut u8;
	fn get_flag_byte(&self) -> &u8;

	fn set_flag_to(&mut self, flag: u8, value: bool) {
		if value {
			self.set_flag(flag)
		} else {
			self.clear_flag(flag)
		}
	}

	fn clear_flag(&mut self, flag: u8) {
		*self.get_flag_byte_mut() &= !flag;
	}

	fn set_flag(&mut self, flag: u8) {
		*self.get_flag_byte_mut() |= flag;
	}

	fn get_flag(&self, flag: u8) -> bool {
		self.get_flag_byte() & flag != 0
	}
}
