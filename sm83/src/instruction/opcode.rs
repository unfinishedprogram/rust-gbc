
#[derive(Clone)]
pub struct Opcode(pub usize, pub usize, pub usize, pub usize, pub usize);

impl From<usize> for Opcode {
	fn from(i: usize) -> Self {
		Opcode(
			(i & 0b11000000) >> 6,
			i & 0b00000111,
			(i & 0b00111000) >> 3,
			(i & 0b00110000) >> 4,
			(i & 0b00001000) >> 3,
		)
	}
}

// lazy_static! {
// 	pub static ref OPCODE_INDEX: Vec<Opcode> = (0..256usize).map(|i| i.into()).collect();
// }

#[inline(always)]
pub fn parse_opcode(raw: u8) -> Opcode {
	Opcode::from(raw as usize)
	// (raw as usize).into()
	// &OPCODE_INDEX[raw as usize]
}
