// https://gb-archive.github.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html#cb

use super::{registers::{Register8, Register16, CompositeU16}, CPU, values::U16Value, decode_tables::DECODE_TABLES};


pub struct Opcode {
	raw:u8,
	x:u8, 
	y:u8, 
	z:u8,
	p:u8, 
	q:u8,
}

impl Opcode {
	pub fn new(raw:u8) -> Opcode {
		let x = (raw & 0b11000000) >> 6; // 
		let y = (raw & 0b00111000) >> 3; //
		let z = (raw & 0b00000111) >> 0; // 
		let p = (raw & 0b00110000) >> 4; //
		let q = (raw & 0b00001000) >> 3; //

		Opcode { raw, x, y, z, p, q, }
	}
}

pub enum Instruction<'a> {
	NOP, 
	STOP, 
	LD(&'a mut dyn U16Value<'a>, Register16),
	JR(Condition, u8),
	ADD(Register16, Register16)
}

pub enum Condition {
	NZ, Z, NC, C, ALWAYS
}
pub enum ALUOperation {
	ADD, ADC, SUB, SBC, AND, XOR, OR, CP
}
pub enum RotShiftOperation {
	RLC, RRC, RL, RR, SLA, SRA, SWAP, SRL
}

pub enum InstructionParameter {
	ValueU16(u8, u8),
	ValueI16(u8, u8),
	ValueU8(u8),
	ValueI8(u8),
	Register8(Register8),
	Register16(Register16),
}

pub fn get_instruction(cpu: &mut CPU, opcode:Opcode) -> Instruction {
	match opcode.x {
		0 => match opcode.z {
			0 => match opcode.y { 
				0 => Instruction::NOP,
				1 => Instruction::LD(cpu.read_nn, Register16::SP),
				2 => Instruction::STOP,
				3 => Instruction::JR(Condition::ALWAYS, *cpu.read_mem()),
				_ => Instruction::JR(DECODE_TABLES.cc[opcode.y], *cpu.read_mem())
			},
			1 => match opcode.q {
				0 => Instruction::LD(DECODE_TABLES.rp[opcode.p], cpu.read_nn()),
				1 => Instruction::ADD(Register16::HL, DECODE_TABLES.rp[opcode.p])
			},
			2 => match opcode.q {
				0 => match opcode.p {
					0 => Instruction::LD(Register16::BC, Register8::A)
				}
				1 => match opcode.p {
					
				}
			}
		},
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn opcode_deconstruction() {
		let opcode = Opcode::new(255);
		assert_eq!(opcode.x, 0b11); 
		assert_eq!(opcode.y, 0b111);
		assert_eq!(opcode.z, 0b111);
		assert_eq!(opcode.p, 0b11);
		assert_eq!(opcode.q, 0b1);
	}
}