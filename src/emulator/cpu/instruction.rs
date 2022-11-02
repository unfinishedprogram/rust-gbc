// Resource
// https://gb-archive.github.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html

pub mod condition;
pub mod decode_tables;
pub mod execute;
pub mod fetch;

#[macro_use]
pub mod mac_instruction;
pub mod opcode;
use condition::Condition;
use core::fmt::Debug;

use super::{
	registers::{CPURegister16, CPURegister8},
	values::{ValueRefI8, ValueRefU16, ValueRefU8},
};

use crate::emulator::flags::InterruptFlag;

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub enum Instruction {
	COMPOSE(Box<Instruction>, Box<Instruction>),
	NOP,
	STOP,
	ERROR(u8),
	LD_8(ValueRefU8, ValueRefU8),
	LDH(ValueRefU8, ValueRefU8),
	LD_16(ValueRefU16, ValueRefU16),
	INC_8(ValueRefU8),
	INC_16(ValueRefU16),
	DEC_8(ValueRefU8),
	DEC_16(ValueRefU16),
	JR(Condition, ValueRefU16),
	ADD_16(ValueRefU16, ValueRefU16),
	ADD_SIGNED(ValueRefU16, ValueRefI8),
	ALU_OP_8(ALUOperation, ValueRefU8, ValueRefU8),
	HALT,
	CALL(Condition, ValueRefU16),
	POP(CPURegister16),
	PUSH(CPURegister16),
	JP(Condition, ValueRefU16),
	// Return
	RET(Condition),
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
	BIT(u8, ValueRefU8),
	RES(u8, ValueRefU8),
	SET(u8, ValueRefU8),
	ROT(RotShiftOperation, ValueRefU8),

	INT(InterruptFlag),

	LD_A_INC_HL,
	LD_A_DEC_HL,
	LD_INC_HL_A,
	LD_DEC_HL_A,
}

#[derive(Copy, Clone)]
pub enum ALUOperation {
	ADD,
	ADC,
	SUB,
	SBC,
	AND,
	XOR,
	OR,
	CP,
}

#[derive(Copy, Clone, Debug)]
pub enum RotShiftOperation {
	RLC,
	RRC,
	RL,
	RR,
	SLA,
	SRA,
	SWAP,
	SRL,
}

impl Debug for ALUOperation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::ADD => write!(f, "add"),
			Self::ADC => write!(f, "adc"),
			Self::SUB => write!(f, "sub"),
			Self::SBC => write!(f, "sbc"),
			Self::AND => write!(f, "and"),
			Self::XOR => write!(f, "xor"),
			Self::OR => write!(f, "or"),
			Self::CP => write!(f, "cp"),
		}
	}
}

impl Debug for Instruction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NOP => write!(f, "nop"),
			Self::STOP => write!(f, "stop"),
			Self::ERROR(arg0) => f.debug_tuple("error").field(arg0).finish(),
			Self::LD_8(arg0, arg1) => write!(f, "ld {arg0:?}, {arg1:?}"),
			Self::LDH(arg0, arg1) => write!(f, "ldh {arg0:?}, {arg1:?}"),
			Self::LD_16(arg0, arg1) => write!(f, "ld {arg0:?}, {arg1:?}"),
			Self::INC_8(arg0) => f.debug_tuple("inc").field(arg0).finish(),
			Self::INC_16(arg0) => write!(f, "inc {arg0:?}"),
			Self::DEC_8(arg0) => write!(f, "dec {arg0:?}"),
			Self::DEC_16(arg0) => write!(f, "dec {arg0:?}"),
			Self::JR(Condition::ALWAYS, arg1) => write!(f, "jr {arg1:?}"),
			Self::JR(arg0, arg1) => write!(f, "jr {arg0:?}, {arg1:?}"),
			Self::ADD_16(arg0, arg1) => f.debug_tuple("add").field(arg0).field(arg1).finish(),
			Self::ADD_SIGNED(arg0, arg1) => {
				f.debug_tuple("ADD_SIGNED").field(arg0).field(arg1).finish()
			}

			Self::ALU_OP_8(a0, a1, a2) => write!(f, "{a0:?} {a1:?}, {a2:?}"),
			Self::HALT => write!(f, "halt"),
			Self::CALL(arg0, arg1) => f.debug_tuple("call").field(arg0).field(arg1).finish(),
			Self::POP(arg0) => f.debug_tuple("pop").field(arg0).finish(),
			Self::PUSH(arg0) => f.debug_tuple("push").field(arg0).finish(),
			Self::JP(Condition::ALWAYS, arg1) => write!(f, "jp {arg1:?}"),
			Self::JP(arg0, arg1) => write!(f, "jp {arg0:?} {arg1:?}"),
			Self::RET(arg0) => f.debug_tuple("ret").field(arg0).finish(),
			Self::RST(arg0) => f.debug_tuple("rst").field(arg0).finish(),
			Self::DI => write!(f, "di"),
			Self::EI => write!(f, "ei"),
			Self::RLCA => write!(f, "rlca"),
			Self::RRCA => write!(f, "rrca"),
			Self::RLA => write!(f, "rla"),
			Self::RRA => write!(f, "rra"),
			Self::DAA => write!(f, "daa"),
			Self::CPL => write!(f, "cpl"),
			Self::SCF => write!(f, "scf"),
			Self::CCF => write!(f, "ccf"),
			Self::BIT(arg0, arg1) => f.debug_tuple("bit").field(arg0).field(arg1).finish(),
			Self::RES(arg0, arg1) => f.debug_tuple("res").field(arg0).field(arg1).finish(),
			Self::SET(arg0, arg1) => f.debug_tuple("set").field(arg0).field(arg1).finish(),
			Self::ROT(arg0, arg1) => f.debug_tuple("rot").field(arg0).field(arg1).finish(),
			Self::INT(arg0) => f.debug_tuple("int").field(arg0).finish(),
			Self::LD_A_DEC_HL => write!(f, "ld a, [hl-]"),
			Self::LD_A_INC_HL => write!(f, "ld a, [hl+]"),
			Self::LD_DEC_HL_A => write!(f, "ld [hl-], a"),
			Self::LD_INC_HL_A => write!(f, "ld [hl+], a"),
			Self::COMPOSE(_, _) => todo!(),
		}
	}
}
