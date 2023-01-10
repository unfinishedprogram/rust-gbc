pub const BIT_0: u8 = 0b00000001;
pub const BIT_1: u8 = 0b00000010;
pub const BIT_2: u8 = 0b00000100;
pub const BIT_3: u8 = 0b00001000;
pub const BIT_4: u8 = 0b00010000;
pub const BIT_5: u8 = 0b00100000;
pub const BIT_6: u8 = 0b01000000;
pub const BIT_7: u8 = 0b10000000;

// Panics if bit > 7
pub fn bit(bit: u8) -> u8 {
	match bit {
		0..8 => 1 << bit,
		_ => unreachable!(),
	}
}
