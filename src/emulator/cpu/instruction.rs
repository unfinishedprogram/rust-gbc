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

use super::{
	registers::{CPURegister16, CPURegister8},
	values::{ValueRefI8, ValueRefU16, ValueRefU8},
	Cpu,
};

use crate::emulator::flags::InterruptFlag;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum Instruction {
	COMPOSE(Box<Instruction>, Box<Instruction>),
	NOP,
	STOP,
	ERROR(u8),
	LD_8(ValueRefU8, ValueRefU8),
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
}

#[derive(Copy, Clone, Debug)]
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
