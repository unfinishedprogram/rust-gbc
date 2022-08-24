use super::registers::Register8;
use super::registers::Register16;
use super::instruction::ALUOperation;
use super::instruction::Condition;
use super::instruction::RotShiftOperation;

struct DecodeTables {
	r:[Register8; 8],
	rp:[Register16; 4],
	rp2:[Register16; 4],
	cc:[Condition; 4],
	alu:[ALUOperation; 8],
	rot:[RotShiftOperation; 8],
}

pub const DECODE_TABLES:DecodeTables = DecodeTables {
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