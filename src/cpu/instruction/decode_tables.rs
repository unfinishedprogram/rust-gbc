use crate::cpu::registers::*;
use crate::cpu::instruction::*;

pub struct DecodeTables {
	pub r:[CPURegister8; 8],
	pub rp:[CPURegister16; 4],
	pub rp2:[CPURegister16; 4],
	pub cc:[Condition; 4],
	pub alu:[ALUOperation; 8],
	pub rot:[RotShiftOperation; 8],
}

pub const DT:DecodeTables = DecodeTables {
	r : [
		CPURegister8::B,
		CPURegister8::C,
		CPURegister8::D,
		CPURegister8::E,
		CPURegister8::H,
		CPURegister8::L,
		CPURegister8::H, // TODO Figure this out
		CPURegister8::A,
	],

	rp : [
		CPURegister16::BC,
		CPURegister16::DE,
		CPURegister16::HL,
		CPURegister16::SP,
	],

	rp2 : [
		CPURegister16::BC,
		CPURegister16::DE,
		CPURegister16::HL,
		CPURegister16::AF,
	],

	cc : [
		Condition::NZ, 
		Condition::Z, 
		Condition::NC, 
		Condition::C, 
	],

	alu : [
		ALUOperation::ADD,
		ALUOperation::ADC,
		ALUOperation::SUB,
		ALUOperation::SBC,
		ALUOperation::AND,
		ALUOperation::XOR,
		ALUOperation::OR,
		ALUOperation::CP,
	],

	rot : [
		RotShiftOperation::RLC,
		RotShiftOperation::RRC,
		RotShiftOperation::RL,
		RotShiftOperation::RR,
		RotShiftOperation::SLA,
		RotShiftOperation::SRA,
		RotShiftOperation::SWAP,
		RotShiftOperation::SRL,
	]
};