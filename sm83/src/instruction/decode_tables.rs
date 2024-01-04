use super::{
	ALUOperation::{self, *},
	CPURegister16,
	CPURegister16::*,
	CPURegister8::*,
	Condition,
	RotShiftOperation::{self, *},
	ValueRefU16,
	ValueRefU8::{self, *},
};

pub struct DecodeTables {
	pub r: [ValueRefU8; 8],
	pub rp: [CPURegister16; 4],
	pub cc: [Condition; 4],
	pub alu: [ALUOperation; 8],
	pub rot: [RotShiftOperation; 8],
}

pub const DT: DecodeTables = DecodeTables {
	r: [
		Reg(B),
		Reg(C),
		Reg(D),
		Reg(E),
		Reg(H),
		Reg(L),
		Mem(ValueRefU16::Reg(HL)),
		Reg(A),
	],

	rp: [BC, DE, HL, SP],
	cc: [Condition::NZ, Condition::Z, Condition::NC, Condition::C],
	alu: [ADD, ADC, SUB, SBC, AND, XOR, OR, CP],
	rot: [RLC, RRC, RL, RR, SLA, SRA, SWAP, SRL],
};
