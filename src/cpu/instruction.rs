// https://gb-archive.github.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html#cb
mod decode_tables;
mod opcode;

use opcode::Opcode;

use self::decode_tables::DT;

use super::{
	registers::{Register8, Register16}, 
	CPU, 
	values::{ValueRefU16, ValueRefI8, ValueRefU8}, 
};

#[allow(non_camel_case_types)]
pub enum Instruction {
	NOP, 
	STOP, 
	ERROR,

	LD_8(ValueRefU8, ValueRefU8),
	LD_16(ValueRefU16, ValueRefU16),

	INC_8(ValueRefU8),
	INC_16(ValueRefU16),

	DEC_8(ValueRefU8),
	DEC_16(ValueRefU16),

	JR(Condition, ValueRefI8),

	ADD_16(ValueRefU16, ValueRefU16),
	ADD_8(ValueRefU8, ValueRefU8),
	
	ALU_OP_8(ALUOperation, ValueRefU8, ValueRefU8),
	ALU_OP_16(ALUOperation, ValueRefU16, ValueRefU16),

	HALT,


	// Accumulator flag ops

	RLCA,
	RRCA,
	RLA,
	RRA,
	DAA,
	CPL,
	SCF,
	CCF,
}
#[derive(Copy, Clone)]
pub enum Condition {
	NZ, Z, NC, C, ALWAYS
}

#[derive(Copy, Clone)]
pub enum ALUOperation {
	ADD, ADC, SUB, SBC, AND, XOR, OR, CP
}
#[derive(Copy, Clone)]
pub enum RotShiftOperation {
	RLC, RRC, RL, RR, SLA, SRA, SWAP, SRL
}

pub fn get_instruction(cpu: &mut CPU, opcode:Opcode) -> Instruction {
	match opcode.x { 
		0 => match opcode.z { // DONE!
			0 => match opcode.y { // Done
				0 => Instruction::NOP, 
				1 => Instruction::LD_16(
					ValueRefU16::Raw(cpu.next_chomp()), 
					ValueRefU16::Reg(Register16::SP)
				), 
				2 => Instruction::STOP, 
				3 => Instruction::JR(
					Condition::ALWAYS, 
					ValueRefI8::Raw(cpu.next_byte() as i8)
				),
				_ => Instruction::JR(
					DT.cc[opcode.y as usize],
					ValueRefI8::Raw(cpu.next_byte() as i8)
				)
			},
			1 => match opcode.q { // Done
				0 => Instruction::LD_16(
					ValueRefU16::Reg(DT.rp[opcode.p as usize]),
					ValueRefU16::Raw(cpu.next_chomp()),
				),
				1 => Instruction::ADD_16(
					ValueRefU16::Reg(Register16::HL),
					ValueRefU16::Reg(DT.rp[opcode.p as usize]),
				),
				_ => Instruction::ERROR
			},
			2 => match opcode.q { // Indirect Loading from memory // TODO ADD DECREMENT
				0 => match opcode.p {
					0 => Instruction::LD_8(
						ValueRefU8::Mem(cpu.read_16(ValueRefU16::Reg(Register16::BC))),
						ValueRefU8::Reg(Register8::A)
					),
					1 => Instruction::LD_8(
						ValueRefU8::Mem(cpu.read_16(ValueRefU16::Reg(Register16::DE))),
						ValueRefU8::Reg(Register8::A)
					),
					2 => Instruction::LD_8( // Increment
						ValueRefU8::Mem(cpu.read_16(ValueRefU16::Reg(Register16::HL))),
						ValueRefU8::Reg(Register8::A)
					),
					3 => Instruction::LD_8( // Decrement 
						ValueRefU8::Mem(cpu.read_16(ValueRefU16::Reg(Register16::HL))),
						ValueRefU8::Reg(Register8::A)
					),
					_ => Instruction::ERROR
				}
				1 => match opcode.p {
					0 => Instruction::LD_8(
						ValueRefU8::Reg(Register8::A), 
						ValueRefU8::Mem(cpu.read_16(ValueRefU16::Reg(Register16::BC))),
					),
					1 => Instruction::LD_8(
						ValueRefU8::Reg(Register8::A), 
						ValueRefU8::Mem(cpu.read_16(ValueRefU16::Reg(Register16::DE))),
					),
					2 => Instruction::LD_8(
						ValueRefU8::Reg(Register8::A), 
						ValueRefU8::Mem(cpu.read_16(ValueRefU16::Reg(Register16::HL))),
					),
					3 => Instruction::LD_8(
						ValueRefU8::Reg(Register8::A), 
						ValueRefU8::Mem(cpu.read_16(ValueRefU16::Reg(Register16::HL))),
					),
					_ => Instruction::ERROR
				},
				_ => Instruction::ERROR
			},
			3 => match opcode.q {
					0 => Instruction::INC_16(ValueRefU16::Reg(DT.rp[opcode.p as usize])), // Increment 16 bit
					1 => Instruction::DEC_16(ValueRefU16::Reg(DT.rp[opcode.p as usize])), // Decrement 16 bit //  TODO DOuble check this
					_ => Instruction::ERROR
			},
			4 => Instruction::INC_8(ValueRefU8::Reg(DT.r[opcode.y as usize])), // Increment 8 bit
			5 => Instruction::DEC_8(ValueRefU8::Reg(DT.r[opcode.y as usize])), // Decrement 8 bit
			6 => Instruction::LD_8(ValueRefU8::Reg(DT.r[opcode.y as usize]), ValueRefU8::Raw(cpu.next_byte())),
			7 => match opcode.y {
				0 => Instruction::RLCA,
				1 => Instruction::RRCA,
				2 => Instruction::RLA,
				3 => Instruction::RRA,
				4 => Instruction::DAA,
				5 => Instruction::CPL,
				6 => Instruction::SCF,
				7 => Instruction::CCF,
				_ => Instruction::ERROR
			}
			_ => Instruction::ERROR
		},
		1 => {
			if opcode.z == 6 && opcode.y == 6 {
				return Instruction::HALT
			} else {
				return Instruction::LD_8 (
					ValueRefU8::Reg(DT.r[opcode.y as usize]),
					ValueRefU8::Reg(DT.r[opcode.z as usize])
				)
			}
		},
		2 => Instruction::ALU_OP_8(DT.alu[opcode.y as usize],ValueRefU8::Reg(Register8::A), ValueRefU8::Reg(DT.r[opcode.z as usize])),
		3 => match opcode.z {
			_ => Instruction::ERROR
		}
		_ => Instruction::ERROR
	}
}