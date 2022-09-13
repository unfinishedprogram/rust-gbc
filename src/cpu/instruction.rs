// Resource
// https://gb-archive.github.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html

mod decode_tables;
pub mod opcode;
pub mod execute;
pub mod mac_instruction;

use opcode::Opcode;

use crate::console_log;
use crate::inst;
use crate::arg;
use crate::mem;
use crate::log;

use self::decode_tables::DT;

use super::{
	registers::{CPURegister8, CPURegister8::*, CPURegister16, CPURegister16::*}, 
	Cpu, 
	values::{ValueRefU16, ValueRefI8, ValueRefU8}, 
};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Instruction {
	NOP, 
	STOP, 
	ERROR(u8),

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

use Instruction::*;

#[derive(Copy, Clone, Debug)]
pub enum Condition {
	NZ, Z, NC, C, ALWAYS
}

#[derive(Copy, Clone, Debug)]
pub enum ALUOperation {
	ADD, ADC, SUB, SBC, AND, XOR, OR, CP
}
#[derive(Copy, Clone, Debug)]
pub enum RotShiftOperation {
	RLC, RRC, RL, RR, SLA, SRA, SWAP, SRL
}

pub fn get_instruction(cpu: &mut Cpu, opcode:Opcode) -> Instruction {
	let x:usize = opcode.x as usize;
	let z:usize = opcode.z as usize;
	let y:usize = opcode.y as usize;
	let p:usize = opcode.p as usize;
	let q:usize = opcode.q as usize;

	console_log!("{:#?}", opcode);

	match (x, z, y, p, q) {
	//(x, z, y, p, q)
		(0, 0, 0, _, _) => inst!(cpu, NOP),
		(0, 0, 1, _, _) => inst!(cpu, LD_16, SP, nn),

		(0, 0, 2, _, _) => inst!(cpu, STOP),
		(0, 0, 3, _, _) => inst!(cpu, JR, (Condition::ALWAYS), d),

		(0, 0, _, _, _) => inst!(cpu, JR, (DT.cc[(y-4)]), d),
		(0, 1, _, _, 0) => inst!(cpu, LD_16, (DT.rp[p]), nn),

		(0, 1, _, _, 1) => inst!(cpu, ADD_16, HL, (DT.rp[p])),

		(0, 2, _, 0, 0) => inst!(cpu, LD_8, [BC]u8, A),
		(0, 2, _, 1, 0) => inst!(cpu, LD_8, [DE]u8, A),

		(0, 2, _, 2, 0) => {
			let inst = inst!(cpu, LD_8, [HL]u8, A);
			cpu.write_16(HL.into(), cpu.read_16(HL.into())+1);
			return inst;
		},
		
		(0, 2, _, 3, 0) => {
			let inst = inst!(cpu, LD_8, [HL]u8, A);
			cpu.write_16(HL.into(), cpu.read_16(HL.into())-1);
			return inst;
		},

		(0, 2, _, 0, 1) => inst!(cpu, LD_8, A, [BC]u8),
		(0, 2, _, 1, 1) => inst!(cpu, LD_8, A, [DE]u8),
		(0, 2, _, 2, 1) => inst!(cpu, LD_8, A, [HL]u8),
		(0, 2, _, 3, 1) => inst!(cpu, LD_8, A, [HL]u8),

		(0, 3, _, _, 0) => inst!(cpu, INC_16, (DT.rp[p])),
		(0, 3, _, _, 1) => inst!(cpu, DEC_16, (DT.rp[p])),
		(0, 4, _, _, _) => inst!(cpu, INC_8, (DT.r[y])),
		(0, 5, _, _, _) => inst!(cpu, DEC_8, (DT.r[y])),
		(0, 6, _, _, _) => inst!(cpu, LD_8, (DT.r[y]), n),

		(0, 7, 0, _, _) => inst!(cpu, RLCA),
		(0, 7, 1, _, _) => inst!(cpu, RRCA),
		(0, 7, 2, _, _) => inst!(cpu, RLA),
		(0, 7, 3, _, _) => inst!(cpu, RRA),
		(0, 7, 4, _, _) => inst!(cpu, DAA),
		(0, 7, 5, _, _) => inst!(cpu, CPL),
		(0, 7, 6, _, _) => inst!(cpu, SCF),
		(0, 7, 7, _, _) => inst!(cpu, CCF),

		(1, 6, 6, _, _) => inst!(cpu, HALT),
		(1, _, _, _, _) => inst!(cpu, LD_8, (DT.r[y]), (DT.r[z])),

		(2, _, _, _, _) => inst!(cpu, ALU_OP_8, (DT.alu[y]), A, (DT.r[z])),
		
		(3, 0, 0, _, _) => inst!(cpu, RET, (DT.cc[0])),
		(3, 0, 1, _, _) => inst!(cpu, RET, (DT.cc[1])),
		(3, 0, 2, _, _) => inst!(cpu, RET, (DT.cc[2])),
		(3, 0, 3, _, _) => inst!(cpu, RET, (DT.cc[3])),

		(3, 0, 4, _, _) => inst!(cpu, LD_8, [(0xFF00 + cpu.next_byte() as u16)]u8, A),

		(3, 0, 5, _, _) => inst!(cpu, ADD_SIGNED, SP, d),

		(3, 0, 6, _, _) => inst!(cpu, LD_8, A, [(0xFF00 + cpu.next_byte() as u16)]u8),

		(3, 0, 7, _, _) => Instruction::LD_16(HL.into(), ValueRefU16::Raw(cpu.read_16(SP.into()).wrapping_add_signed(cpu.next_displacement() as i16))),
		
		(3, 1, _, _, 0) => inst!(cpu, POP, (DT.rp2[p])),

		(3, 1, _, 0, 1) => inst!(cpu, RET, (Condition::ALWAYS)),
		(3, 1, _, 1, 1) => inst!(cpu, RETI),
		(3, 1, _, 2, 1) => inst!(cpu, JP, (Condition::ALWAYS), HL),
		(3, 1, _, 3, 1) => inst!(cpu, LD_16, SP, HL),

		(3, 2, 0, _, _) => inst!(cpu, JP, (DT.cc[0]), nn),
		(3, 2, 1, _, _) => inst!(cpu, JP, (DT.cc[1]), nn),
		(3, 2, 2, _, _) => inst!(cpu, JP, (DT.cc[2]), nn),
		(3, 2, 3, _, _) => inst!(cpu, JP, (DT.cc[3]), nn),

		(3, 2, 4, _, _) => inst!(cpu, LD_8, [(0xFF00 + cpu.read_8(C.into()) as u16)]u8, A),
		(3, 2, 5, _, _) => inst!(cpu, LD_8, [nn]u8, A),

		(3, 2, 6, _, _) => inst!(cpu, LD_8, A, [(0xFF00 + cpu.read_8(C.into()) as u16)]u8),
		(3, 2, 7, _, _) => inst!(cpu, LD_8, A, [nn]u8),


		(3, 3, 0, _, _) => inst!(cpu, JP, (Condition::ALWAYS), nn),

		(3, 3, 1, _, _) => {
			let cb_opcode = Opcode::from(cpu.next_byte());
			match cb_opcode.x {
				0 => inst!(cpu, ROT, (DT.rot[cb_opcode.y as usize]), (DT.r[cb_opcode.z as usize])),
				1 => inst!(cpu, BIT, (cb_opcode.y), (DT.r[cb_opcode.z as usize])),
				2 => inst!(cpu, RES, (cb_opcode.y), (DT.r[cb_opcode.z as usize])),
				3 => inst!(cpu, SET, (cb_opcode.y), (DT.r[cb_opcode.z as usize])),
				_ => inst!(cpu, ERROR, (cb_opcode.raw)),
			}
		},
		
		(3, 3, 6, _, _) => inst!(cpu, DI),
		(3, 3, 7, _, _) => inst!(cpu, EI),

		(3, 4, 0, _, _) => inst!(cpu, CALL, (DT.cc[0]), nn),
		(3, 4, 1, _, _) => inst!(cpu, CALL, (DT.cc[1]), nn),
		(3, 4, 2, _, _) => inst!(cpu, CALL, (DT.cc[2]), nn),
		(3, 4, 3, _, _) => inst!(cpu, CALL, (DT.cc[3]), nn),

		(3, 5, _, _, 0) => inst!(cpu, PUSH, (DT.rp2[p])),
		(3, 5, _, 0, 1) => inst!(cpu, CALL, (Condition::ALWAYS), nn),

		(3, 6, _, _, _) => inst!(cpu, ALU_OP_8, (DT.alu[y]), A, n),
		(3, 7, _, _, _) => inst!(cpu, RST, ((opcode.y as u16) * 8)),
		(_, _, _, _, _) => inst!(cpu, ERROR, (opcode.raw)),
	}
}