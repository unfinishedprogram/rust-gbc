pub mod cpu {
	use crate::bits::*;

	pub const Z: u8 = BIT_0;
	pub const N: u8 = BIT_1;
	pub const H: u8 = BIT_2;
	pub const C: u8 = BIT_3;
}

// Interrupt Flag Bits
pub mod interrupt {
	use crate::bits::*;

	pub const V_BLANK: u8 = BIT_0;
	pub const LCD_STAT: u8 = BIT_1;
	pub const TIMER: u8 = BIT_2;
	pub const SERIAL: u8 = BIT_3;
	pub const JOY_PAD: u8 = BIT_4;
}

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
