use lazy_static::lazy_static;

#[derive(Clone)]
pub struct Opcode(pub usize, pub usize, pub usize, pub usize, pub usize);

lazy_static! {
	pub static ref OPCODE_INDEX: Vec<Opcode> = {
		(0..256)
			.map(|i| {
				Opcode(
					(i & 0b11000000) >> 6,
					i & 0b00000111,
					(i & 0b00111000) >> 3,
					(i & 0b00110000) >> 4,
					(i & 0b00001000) >> 3,
				)
			})
			.collect()
	};
}

pub fn parse_opcode(raw: u8) -> &'static Opcode {
	&OPCODE_INDEX[raw as usize]
}