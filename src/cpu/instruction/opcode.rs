#[derive(Debug)]
pub struct Opcode {
	pub x: u8,
	pub z: u8,
	pub y: u8,
	pub p: u8,
	pub q: u8,
	pub raw: u8,
}

impl Opcode {
	pub fn from(raw: u8) -> Opcode {
		let x = (raw & 0b11000000) >> 6;
		let y = (raw & 0b00111000) >> 3;
		let z = (raw & 0b00000111) >> 0;
		let p = (raw & 0b00110000) >> 4;
		let q = (raw & 0b00001000) >> 3;

		Opcode { x, z, y, p, q, raw }
	}
}
