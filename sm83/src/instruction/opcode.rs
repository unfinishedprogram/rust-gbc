pub struct Opcode(pub usize, pub usize, pub usize, pub usize, pub usize);

pub fn parse_opcode(raw: u8) -> Opcode {
	let i = raw as usize;
	let x = (i >> 6) & 0b11;
	let z = i & 0b111;
	let y = (i >> 3) & 0b111;
	let p = (i >> 4) & 0b11;
	let q = (i >> 3) & 0b1;

	Opcode(x, z, y, p, q)
}
