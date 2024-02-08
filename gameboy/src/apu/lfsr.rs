use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Lfsr {
	pub shift_register: u16,
	pub width: u16,
}

const SHIFT_REGISTER_INITIAL: u16 = 0x7FFF;

impl Default for Lfsr {
	fn default() -> Self {
		Self {
			shift_register: SHIFT_REGISTER_INITIAL,
			width: 14,
		}
	}
}

impl Lfsr {
	pub fn reset(&mut self) {
		self.shift_register = SHIFT_REGISTER_INITIAL;
	}

	pub fn step(&mut self) -> bool {
		let bit_0 = self.shift_register & 1 != 0;
		let bit_1 = self.shift_register & 2 != 0;
		let xor = bit_0 ^ bit_1;

		self.shift_register >>= 1;

		let width_bit = 1 << self.width;

		self.shift_register &= !width_bit;
		self.shift_register |= (xor as u16) << self.width;

		!bit_0
	}
}
