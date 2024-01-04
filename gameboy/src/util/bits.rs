pub const BIT_0: u8 = 0b00000001;
pub const BIT_1: u8 = 0b00000010;
pub const BIT_2: u8 = 0b00000100;
pub const BIT_3: u8 = 0b00001000;
pub const BIT_4: u8 = 0b00010000;
pub const BIT_5: u8 = 0b00100000;
pub const BIT_6: u8 = 0b01000000;
pub const BIT_7: u8 = 0b10000000;

pub fn interleave(a: u8, b: u8) -> u16 {
	let a = a as u16;
	let b = b as u16;

	let a = (a ^ (a << 4)) & 0x0f0f;
	let b = (b ^ (b << 4)) & 0x0f0f;

	let a = (a ^ (a << 2)) & 0x3333;
	let b = (b ^ (b << 2)) & 0x3333;

	let a = (a ^ (a << 1)) & 0x5555;
	let b = (b ^ (b << 1)) & 0x5555;

	b << 1 | a
}
