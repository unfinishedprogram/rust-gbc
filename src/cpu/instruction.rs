pub struct Instruction {
	bytes:u8,
	cycles:u8,
	opcode:u8,
}

// https://gb-archive.github.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html#cb

pub struct Opcode {
	raw:u8,
	x:u8, 
	y:u8, 
	z:u8,
	p:u8, 
	q:u8,
}

impl Opcode {
	fn new(raw:u8) -> Opcode {
		let x = 0;

		Opcode {
			raw,
		}
	}
}