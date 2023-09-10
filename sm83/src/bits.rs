#![allow(dead_code)]
pub const BIT_0: u8 = 0b00000001;
pub const BIT_1: u8 = 0b00000010;
pub const BIT_2: u8 = 0b00000100;
pub const BIT_3: u8 = 0b00001000;
pub const BIT_4: u8 = 0b00010000;
pub const BIT_5: u8 = 0b00100000;
pub const BIT_6: u8 = 0b01000000;
pub const BIT_7: u8 = 0b10000000;

pub trait IntUtils {
	fn activate_rightmost_zeros(self) -> Self;
	fn test_add_carry_bit(bit: usize, a: Self, b: Self) -> bool;
}

impl IntUtils for u16 {
	fn activate_rightmost_zeros(self) -> Self {
		let x = self;
		x | x.wrapping_sub(1)
	}

	fn test_add_carry_bit(bit: usize, a: Self, b: Self) -> bool {
		let mask = (1 << bit).activate_rightmost_zeros();
		(a & mask) + (b & mask) > mask
	}
}
