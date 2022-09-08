use crate::cpu::registers::*;
use crate::cpu::instruction::*;

pub struct DecodeTables {
	pub r:[Register8; 8],
	pub rp:[Register16; 4],
	pub rp2:[Register16; 4],
	pub cc:[Condition; 4],
	pub alu:[ALUOperation; 8],
	pub rot:[RotShiftOperation; 8],
}

pub const DT:DecodeTables = DecodeTables {
	r : [
		Register8::B,
		Register8::C,
		Register8::D,
		Register8::E,
		Register8::H,
		Register8::L,
		Register8::H, // TODO Figure this out
		Register8::A,
	],

	rp : [
		Register16::BC,
		Register16::DE,
		Register16::HL,
		Register16::SP,
	],

	rp2 : [
		Register16::BC,
		Register16::DE,
		Register16::HL,
		Register16::AF,
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