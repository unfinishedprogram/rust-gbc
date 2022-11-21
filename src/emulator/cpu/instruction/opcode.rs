pub type Opcode = (usize, usize, usize, usize, usize);

pub fn parse_opcode(raw: u8) -> Opcode {
	let x = ((raw & 0b11000000) >> 6) as usize;
	let y = ((raw & 0b00111000) >> 3) as usize;
	let z = (raw & 0b00000111) as usize;
	let p = ((raw & 0b00110000) >> 4) as usize;
	let q = ((raw & 0b00001000) >> 3) as usize;
	(x, z, y, p, q)
}
