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
	RETI,
	RET(Condition),
	RST(ValueRefU16),
	DI,
	EI,
	LD_HL_SP_DD(ValueRefI8),
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

	INT(u8),

	LD_A_INC_HL,
	LD_A_DEC_HL,
	LD_INC_HL_A,
	LD_DEC_HL_A,
}

#[derive(Clone)]
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

#[derive(Clone)]
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

impl Debug for RotShiftOperation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::RLC => write!(f, "rlc"),
			Self::RRC => write!(f, "rrc"),
			Self::RL => write!(f, "rl"),
			Self::RR => write!(f, "rr"),
			Self::SLA => write!(f, "sla"),
			Self::SRA => write!(f, "sra"),
			Self::SWAP => write!(f, "swap"),
			Self::SRL => write!(f, "srl"),
		}
	}
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
			Self::INC_8(arg0) => write!(f, "inc {arg0:?}"),
			Self::INC_16(arg0) => write!(f, "inc {arg0:?}"),
			Self::DEC_8(arg0) => write!(f, "dec {arg0:?}"),
			Self::DEC_16(arg0) => write!(f, "dec {arg0:?}"),
			Self::JR(Condition::ALWAYS, arg1) => write!(f, "jr {arg1:?}"),
			Self::JR(arg0, arg1) => write!(f, "jr {arg0:?}, {arg1:?}"),
			Self::ADD_16(arg0, arg1) => write!(f, "add {arg0:?}, {arg1:?}"),
			Self::ADD_SIGNED(arg0, arg1) => write!(f, "add {arg0:?}, {arg1:?}"),
			Self::ALU_OP_8(a0, a1, a2) => write!(f, "{a0:?} {a1:?}, {a2:?}"),
			Self::HALT => write!(f, "halt"),
			Self::CALL(Condition::ALWAYS, arg1) => write!(f, "call {arg1:?}"),
			Self::CALL(arg0, arg1) => write!(f, "call {arg0:?}, {arg1:?}"),
			Self::POP(arg0) => write!(f, "pop {arg0:?}"),
			Self::PUSH(arg0) => write!(f, "push {arg0:?}"),
			Self::JP(Condition::ALWAYS, arg1) => write!(f, "jp {arg1:?}"),
			Self::JP(arg0, arg1) => write!(f, "jp {arg0:?}, {arg1:?}"),
			Self::RET(Condition::ALWAYS) => write!(f, "ret"),
			Self::RET(arg0) => write!(f, "ret {arg0:?}"),
			Self::RST(ValueRefU16::Raw(arg0)) => write!(f, "rst ${:02X}", *arg0 as u8),
			Self::RST(arg0) => write!(f, "rst {arg0:?}"),
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
			Self::BIT(arg0, arg1) => write!(f, "bit {arg0}, {arg1:?}"),
			Self::RES(arg0, arg1) => write!(f, "res {arg0}, {arg1:?}"),
			Self::SET(arg0, arg1) => write!(f, "set {arg0}, {arg1:?}"),
			Self::ROT(arg0, arg1) => write!(f, "{arg0:?} {arg1:?}"),
			Self::INT(arg0) => f.debug_tuple("int").field(arg0).finish(),
			Self::LD_A_DEC_HL => write!(f, "ld a, [hl-]"),
			Self::LD_A_INC_HL => write!(f, "ld a, [hl+]"),
			Self::LD_DEC_HL_A => write!(f, "ld [hl-], a"),
			Self::LD_INC_HL_A => write!(f, "ld [hl+], a"),
			Self::LD_HL_SP_DD(arg0) => write!(f, "ld hl, sp + {arg0:?}"),
			Self::COMPOSE(a, b) => write!(f, "{a:?} COMP {b:?}"),
			Self::RETI => write!(f, "reti"),
		}
	}
}
