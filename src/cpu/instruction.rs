// Resource
// https://gb-archive.github.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html

mod decode_tables;
pub mod opcode;
pub mod execute;

use opcode::Opcode;

use self::decode_tables::DT;

use super::{
	registers::{Register8, Register8::*, Register16, Register16::*}, 
	Cpu, 
	values::{ValueRefU16, ValueRefI8, ValueRefU8}, 
};

#[allow(non_camel_case_types)]
pub enum Instruction {
	NOP, 
	STOP, 
	ERROR,

	LD_8(ValueRefU8, ValueRefU8),
	LDD_8(ValueRefU8, ValueRefU8),
	LDI_8(ValueRefU8, ValueRefU8),
	LD_16(ValueRefU16, ValueRefU16),

	INC_8(ValueRefU8),
	INC_16(ValueRefU16),

	DEC_8(ValueRefU8),
	DEC_16(ValueRefU16),

	JR(Condition, ValueRefI8),

	ADD_16(ValueRefU16, ValueRefU16),

	ADD_SIGNED(ValueRefU16, ValueRefI8),
	
	ALU_OP_8(ALUOperation, ValueRefU8, ValueRefU8),

	HALT,

	CALL(Condition, ValueRefU16),

	POP(Register16),
	PUSH(Register16),

	JP(Condition, ValueRefU16),

	// Return
	RET(Condition),
	RETI,

	RST(ValueRefU16),

	DI,
	EI,

	// Accumulator flag ops

	RLCA,
	RRCA,
	RLA,
	RRA,
	DAA,
	CPL,
	SCF,
	CCF,

	//  CB Instructions
	BIT(u8, Register8), 
	RES(u8, Register8), 
	SET(u8, Register8), 
	ROT(RotShiftOperation, Register8)
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

pub fn get_instruction(cpu: &mut Cpu, opcode:Opcode) -> Instruction {
	match opcode.x { 
		0 => match opcode.z {
			0 => match opcode.y {
				0 => Instruction::NOP, 
				1 => Instruction::LD_16(
					ValueRefU16::Raw(cpu.next_chomp()), 
					SP.into()
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
			1 => match opcode.q {
				0 => Instruction::LD_16(
					DT.rp[opcode.p as usize].into(),
					ValueRefU16::Raw(cpu.next_chomp()),
				),
				1 => Instruction::ADD_16(
					HL.into(),
					DT.rp[opcode.p as usize].into(),
				),
				_ => Instruction::ERROR
			},
			2 => match opcode.q { // Indirect Loading from memory // TODO ADD DECREMENT
				0 => match opcode.p {
					0 => Instruction::LD_8(
						ValueRefU8::Mem(cpu.read_16(BC.into())),
						A.into()
					),
					1 => Instruction::LD_8(
						ValueRefU8::Mem(cpu.read_16(DE.into())),
						A.into()
					),
					2 => Instruction::LDI_8( // Increment
						ValueRefU8::Mem(cpu.read_16(HL.into())),
						A.into()
					),
					3 => Instruction::LDD_8( // Decrement 
						ValueRefU8::Mem(cpu.read_16(HL.into())),
						A.into()
					),
					_ => Instruction::ERROR
				}
				1 => match opcode.p {
					0 => Instruction::LD_8(
						A.into(), 
						ValueRefU8::Mem(cpu.read_16(BC.into())),
					),
					1 => Instruction::LD_8(
						A.into(), 
						ValueRefU8::Mem(cpu.read_16(DE.into())),
					),
					2 => Instruction::LD_8(
						A.into(), 
						ValueRefU8::Mem(cpu.read_16(HL.into())),
					),
					3 => Instruction::LD_8(
						A.into(), 
						ValueRefU8::Mem(cpu.read_16(HL.into())),
					),
					_ => Instruction::ERROR
				},
				_ => Instruction::ERROR
			},
			3 => match opcode.q {
					0 => Instruction::INC_16(DT.rp[opcode.p as usize].into()), // Increment 16 bit
					1 => Instruction::DEC_16(DT.rp[opcode.p as usize].into()), // Decrement 16 bit //  TODO DOuble check this
					_ => Instruction::ERROR
			},
			4 => Instruction::INC_8(DT.r[opcode.y as usize].into()), // Increment 8 bit
			5 => Instruction::DEC_8(DT.r[opcode.y as usize].into()), // Decrement 8 bit
			6 => Instruction::LD_8(DT.r[opcode.y as usize].into(), cpu.next_byte().into()),
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
					DT.r[opcode.y as usize].into(),
					DT.r[opcode.z as usize].into()
				)
			}
		},
		2 => Instruction::ALU_OP_8(DT.alu[opcode.y as usize], A.into(), DT.r[opcode.z as usize].into()),
		3 => match opcode.z {
			0 => match opcode.y {
				0..=3 => Instruction::RET(DT.cc[opcode.y as usize]),
				4 => Instruction::LD_8(ValueRefU8::Mem(0xFF00 + cpu.next_byte() as u16), A.into()),
				5 => Instruction::ADD_SIGNED(SP.into(), ValueRefI8::Raw(cpu.next_displacement())),
				6 => Instruction::LD_8(A.into(), ValueRefU8::Mem(0xFF00 + cpu.next_byte() as u16)),
				7 => Instruction::LD_16(
					HL.into(), 
					ValueRefU16::Raw(
						cpu.read_16(
							SP.into())
							.wrapping_add_signed(cpu.next_displacement() as i16)
					)),
				_ => Instruction::ERROR,
			}
			1 => match opcode.q {
				0 => Instruction::POP(DT.rp2[opcode.p as usize]),
				1 => match opcode.p {
					0 => Instruction::RET(Condition::ALWAYS),
					1 => Instruction::RETI,
					2 => Instruction::JP(Condition::ALWAYS, HL.into()),
					3 => Instruction::LD_16(SP.into(), HL.into()),
					_ => Instruction::ERROR,
				}
				_ => Instruction::ERROR,
			}
			2 => match opcode.y {
				0..=3 => Instruction::JP(DT.cc[opcode.y as usize], 
					cpu.next_chomp().into()
				),
				4 => Instruction::LD_8(ValueRefU8::Mem(0xFF00 + cpu.read_8(C.into()) as u16), A.into()),
				5 => Instruction::LD_8(ValueRefU8::Mem(cpu.next_chomp()), A.into()),
				6 => Instruction::LD_8(A.into(), ValueRefU8::Mem(0xFF00 + cpu.read_8(C.into()) as u16)),
				7 => Instruction::LD_8(A.into(), ValueRefU8::Mem(cpu.next_chomp())),
				_ => Instruction::ERROR,
			}
			3 => match opcode.y {
				0 => Instruction::JP(Condition::ALWAYS, ValueRefU16::Raw(cpu.next_chomp())),
				1 =>  {
					let cb_opcode = Opcode::from(cpu.next_byte());
					match cb_opcode.x {
						0 => Instruction::ROT(DT.rot[cb_opcode.y as usize], DT.r[cb_opcode.z as usize]),
						1 => Instruction::BIT(cb_opcode.y, DT.r[cb_opcode.z as usize]),
						2 => Instruction::RES(cb_opcode.y, DT.r[cb_opcode.z as usize]),
						3 => Instruction::SET(cb_opcode.y, DT.r[cb_opcode.z as usize]),
						_ => Instruction::ERROR,
					}
				},
				6 => Instruction::DI,
				7 => Instruction::EI,
				_ => Instruction::ERROR,
			}
			4 => match opcode.y {
				0..=3 => Instruction::CALL(DT.cc[opcode.y as usize], ValueRefU16::Raw(cpu.next_chomp())),
				_ => Instruction::ERROR,
			}
			5 => match opcode.q {
				0 => Instruction::PUSH(DT.rp2[opcode.p as usize]),
				1 => match opcode.p {
					0 => Instruction::CALL(Condition::ALWAYS, ValueRefU16::Raw(cpu.next_chomp())),
					_ => Instruction::ERROR,
				}
				_ => Instruction::ERROR,
			}
			6 => Instruction::ALU_OP_8(DT.alu[opcode.y as usize], A.into(), ValueRefU8::Raw(cpu.next_byte())),
			7 => Instruction::RST(ValueRefU16::Raw((opcode.y as u16) * 8)),
			_ => Instruction::ERROR,
 		}
		_ => Instruction::ERROR
	}
}