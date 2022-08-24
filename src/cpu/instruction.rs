// https://gb-archive.github.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html#cb

use super::{registers::{Register8, Register16, CompositeU16}, CPU, values::U16Value};


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
	JR,
}

pub enum Condition {
	NZ, Z, NC, C
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
				1 => Instruction::LD(&mut CompositeU16(cpu.fetch_next_byte_from_memory(), cpu.fetch_next_byte_from_memory()), Register16::SP),
				2 => Instruction::STOP,
			},
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