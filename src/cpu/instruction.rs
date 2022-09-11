// Resource
// https://gb-archive.github.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html

mod decode_tables;
pub mod opcode;
pub mod execute;

use opcode::Opcode;

use self::decode_tables::DT;

use super::{
	registers::{CPURegister8, CPURegister8::*, CPURegister16, CPURegister16::*}, 
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

	POP(CPURegister16),
	PUSH(CPURegister16),

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
	BIT(u8, CPURegister8), 
	RES(u8, CPURegister8), 
	SET(u8, CPURegister8), 
	ROT(RotShiftOperation, CPURegister8)
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
	let x = opcode.x;
	let z = opcode.z;
	let y = opcode.y;
	let p = opcode.p;
	let q = opcode.q;

	match (x, z, y, p, q) {
	//(x, z, y, p, q)
		(0, 0, 0, _, _) => Instruction::NOP,
		(0, 0, 1, _, _) => Instruction::LD_16 (cpu.next_chomp().into(), SP.into()),
		(0, 0, 2, _, _) => Instruction::STOP,
		(0, 0, 3, _, _) => Instruction::JR(Condition::ALWAYS, (cpu.next_byte() as i8).into()),
		(0, 0, _, _, _) => Instruction::JR(DT.cc[y as usize], (cpu.next_byte() as i8).into()),

		(0, 1, _, _, 0) => Instruction::LD_16 (DT.rp[p as usize].into(), cpu.next_chomp().into()),
		(0, 1, _, _, 1) => Instruction::ADD_16(HL.into(), DT.rp[p as usize].into()),

		(0, 2, _, 0, 0) => Instruction::LD_8 (ValueRefU8::Mem(cpu.read_16(BC.into())), A.into()),
		(0, 2, _, 0, 1) => Instruction::LD_8 (ValueRefU8::Mem(cpu.read_16(DE.into())), A.into()),
		(0, 2, _, 0, 2) => Instruction::LDI_8(ValueRefU8::Mem(cpu.read_16(HL.into())), A.into()),
		(0, 2, _, 0, 3) => Instruction::LDD_8(ValueRefU8::Mem(cpu.read_16(HL.into())), A.into()),

		(0, 2, _, 1, 0) => Instruction::LD_8 (A.into(), ValueRefU8::Mem(cpu.read_16(BC.into()))),
		(0, 2, _, 1, 1) => Instruction::LD_8 (A.into(), ValueRefU8::Mem(cpu.read_16(DE.into()))),
		(0, 2, _, 1, 2) => Instruction::LD_8 (A.into(), ValueRefU8::Mem(cpu.read_16(HL.into()))),
		(0, 2, _, 1, 3) => Instruction::LD_8 (A.into(), ValueRefU8::Mem(cpu.read_16(HL.into()))),

		(0, 3, _, _, 0) => Instruction::INC_16(DT.rp[p as usize].into()),
		(0, 3, _, _, 1) => Instruction::DEC_16(DT.rp[p as usize].into()),
		(0, 4, _, _, _) => Instruction::INC_8 (DT.r[y as usize].into()),
		(0, 5, _, _, _) => Instruction::DEC_8 (DT.r[y as usize].into()),
		(0, 6, _, _, _) => Instruction::LD_8  (DT.r[y as usize].into(), cpu.next_byte().into()),

		(0, 7, 0, _, _) => Instruction::RLCA,
		(0, 7, 1, _, _) => Instruction::RRCA,
		(0, 7, 2, _, _) => Instruction::RLA,
		(0, 7, 3, _, _) => Instruction::RRA,
		(0, 7, 4, _, _) => Instruction::DAA,
		(0, 7, 5, _, _) => Instruction::CPL,
		(0, 7, 6, _, _) => Instruction::SCF,
		(0, 7, 7, _, _) => Instruction::CCF,

		(1, 6, 6, _, _) => Instruction::HALT,
		(1, _, _, _, _) => Instruction::LD_8 (DT.r[y as usize].into(), DT.r[z as usize].into()),

		(2, _, _, _, _) => Instruction::ALU_OP_8(DT.alu[y as usize], A.into(), DT.r[z as usize].into()),
		
		(3, 0, 0, _, _) => Instruction::RET(DT.cc[0]),
		(3, 0, 1, _, _) => Instruction::RET(DT.cc[1]),
		(3, 0, 2, _, _) => Instruction::RET(DT.cc[2]),
		(3, 0, 3, _, _) => Instruction::RET(DT.cc[3]),

		(3, 0, 4, _, _) => Instruction::LD_8(ValueRefU8::Mem(0xFF00 + cpu.next_byte() as u16), A.into()),
		(3, 0, 5, _, _) => Instruction::ADD_SIGNED(SP.into(), ValueRefI8::Raw(cpu.next_displacement())),
		(3, 0, 6, _, _) => Instruction::LD_8(A.into(), ValueRefU8::Mem(0xFF00 + cpu.next_byte() as u16)),
		(3, 0, 7, _, _) => Instruction::LD_16(HL.into(), ValueRefU16::Raw(cpu.read_16(SP.into()).wrapping_add_signed(cpu.next_displacement() as i16))),

		(3, 1, _, _, 0) => Instruction::POP(DT.rp2[opcode.p as usize]),

		(3, 1, _, 0, 1) => Instruction::RET(Condition::ALWAYS),
		(3, 1, _, 1, 1) => Instruction::RETI,
		(3, 1, _, 2, 1) => Instruction::JP(Condition::ALWAYS, HL.into()),
		(3, 1, _, 3, 1) => Instruction::LD_16(SP.into(), HL.into()),

		(3, 2, 0, _, _) => Instruction::JP(DT.cc[0], cpu.next_chomp().into()),
		(3, 2, 1, _, _) => Instruction::JP(DT.cc[1], cpu.next_chomp().into()),
		(3, 2, 2, _, _) => Instruction::JP(DT.cc[2], cpu.next_chomp().into()),
		(3, 2, 3, _, _) => Instruction::JP(DT.cc[3], cpu.next_chomp().into()),

		(3, 2, 4, _, _) => Instruction::LD_8(ValueRefU8::Mem(0xFF00 + cpu.read_8(C.into()) as u16), A.into()),
		(3, 2, 5, _, _) => Instruction::LD_8(ValueRefU8::Mem(cpu.next_chomp()), A.into()),
		(3, 2, 6, _, _) => Instruction::LD_8(A.into(), ValueRefU8::Mem(0xFF00 + cpu.read_8(C.into()) as u16)),
		(3, 2, 7, _, _) => Instruction::LD_8(A.into(), ValueRefU8::Mem(cpu.next_chomp())),

		(3, 3, 0, _, _) => Instruction::JP(Condition::ALWAYS, ValueRefU16::Raw(cpu.next_chomp())),
		(3, 3, 1, _, _) => {
			let cb_opcode = Opcode::from(cpu.next_byte());
			match cb_opcode.x {
				0 => Instruction::ROT(DT.rot[cb_opcode.y as usize], DT.r[cb_opcode.z as usize]),
				1 => Instruction::BIT(cb_opcode.y, DT.r[cb_opcode.z as usize]),
				2 => Instruction::RES(cb_opcode.y, DT.r[cb_opcode.z as usize]),
				3 => Instruction::SET(cb_opcode.y, DT.r[cb_opcode.z as usize]),
				_ => Instruction::ERROR,
			}
		},
		(3, 3, 6, _, _) => Instruction::DI,
		(3, 3, 7, _, _) => Instruction::EI,

		(3, 4, 0, _, _) => Instruction::CALL(DT.cc[0], cpu.next_chomp().into()),
		(3, 4, 1, _, _) => Instruction::CALL(DT.cc[1], cpu.next_chomp().into()),
		(3, 4, 2, _, _) => Instruction::CALL(DT.cc[2], cpu.next_chomp().into()),
		(3, 4, 3, _, _) => Instruction::CALL(DT.cc[3], cpu.next_chomp().into()),

		(3, 5, 0, _, 0) => Instruction::PUSH(DT.rp2[p as usize]),
		(3, 5, 0, 0, 1) => Instruction::CALL(Condition::ALWAYS, cpu.next_chomp().into()),

		(3, 6, _, _, _) => Instruction::ALU_OP_8(DT.alu[y as usize], A.into(), cpu.next_byte().into()),
		(3, 7, _, _, _) => Instruction::RST(((opcode.y as u16) * 8).into()),
		(_, _, _, _, _) => Instruction::ERROR,
	}
}