pub type Opcode = (usize, usize, usize, usize, usize);

pub fn parse_opcode(raw: u8) -> Opcode {
	let raw = raw as usize;
	(
		((raw & 0b11000000) >> 6),
		(raw & 0b00000111),
		((raw & 0b00111000) >> 3),
		((raw & 0b00110000) >> 4),
		((raw & 0b00001000) >> 3),
	)
}
