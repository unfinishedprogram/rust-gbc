use super::{
	decode_tables::DT, opcode::Opcode, CPURegister16::*, CPURegister8::*, Condition, Instruction,
	Instruction::*, ValueRefU8,
};

use crate::emulator::cpu::CPU;
use crate::emulator::state::EmulatorState;

use crate::{arg, inst, mem}; // Macros

pub fn fetch_instruction(cpu: &mut EmulatorState) -> Instruction {
	let opcode = Opcode::from(cpu.next_byte());

	let x = opcode.x as usize;
	let z = opcode.z as usize;
	let y = opcode.y as usize;
	let p = opcode.p as usize;
	let q = opcode.q as usize;

	match (x, z, y, p, q) {
		//(x, z, y, p, q)
		(0, 0, 0, _, _) => inst!(cpu, NOP),
		(0, 0, 1, _, _) => inst!(cpu, LD_16, SP, nn),

		(0, 0, 2, _, _) => inst!(cpu, STOP),
		(0, 0, 3, _, _) => inst!(cpu, JR, (Condition::ALWAYS), d),

		(0, 0, _, _, _) => inst!(cpu, JR, (DT.cc[(y - 4)]), d),
		(0, 1, _, _, 0) => inst!(cpu, LD_16, (DT.rp[p]), nn),

		(0, 1, _, _, 1) => inst!(cpu, ADD_16, HL, (DT.rp[p])),

		(0, 2, _, 0, 0) => inst!(cpu, LD_8, [BC]u8, A),
		(0, 2, _, 1, 0) => inst!(cpu, LD_8, [DE]u8, A),

		(0, 2, _, 2, 0) => Instruction::COMPOSE(
			inst!(cpu, LD_8, [HL]u8, A).into(),
			inst!(cpu, INC_16, HL).into(),
		),

		(0, 2, _, 3, 0) => Instruction::COMPOSE(
			inst!(cpu, LD_8, [HL]u8, A).into(),
			inst!(cpu, DEC_16, HL).into(),
		),

		(0, 2, _, 0, 1) => inst!(cpu, LD_8, A, [BC]u8),
		(0, 2, _, 1, 1) => inst!(cpu, LD_8, A, [DE]u8),

		(0, 2, _, 2, 1) => Instruction::COMPOSE(
			inst!(cpu, LD_8, A, [HL]u8).into(),
			inst!(cpu, INC_16, HL).into(),
		),

		(0, 2, _, 3, 1) => Instruction::COMPOSE(
			inst!(cpu, LD_8, A, [HL]u8).into(),
			inst!(cpu, DEC_16, HL).into(),
		),

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

		(3, 0, 7, _, _) => Instruction::COMPOSE(
			inst!(cpu, LD_16, HL, SP).into(),
			inst!(cpu, ADD_SIGNED, HL, d).into(),
		),

		(3, 1, _, _, 0) => inst!(cpu, POP, (DT.rp2[p])),

		(3, 1, _, 0, 1) => inst!(cpu, RET, (Condition::ALWAYS)),
		(3, 1, _, 1, 1) => Instruction::COMPOSE(
			inst!(cpu, RET, (Condition::ALWAYS)).into(),
			inst!(cpu, EI).into(),
		),
		(3, 1, _, 2, 1) => inst!(cpu, JP, (Condition::ALWAYS), HL),
		(3, 1, _, 3, 1) => inst!(cpu, LD_16, SP, HL),

		(3, 2, 4, _, _) => inst!(cpu, LD_8, [(0xFF00 + cpu.read_8(C.into()) as u16)]u8, A),
		(3, 2, 5, _, _) => inst!(cpu, LD_8, [nn]u8, A),

		(3, 2, 6, _, _) => inst!(cpu, LD_8, A, [(0xFF00 + cpu.read_8(C.into()) as u16)]u8),
		(3, 2, 7, _, _) => inst!(cpu, LD_8, A, [nn]u8),

		(3, 2, c, _, _) => inst!(cpu, JP, (DT.cc[c]), nn),

		(3, 3, 0, _, _) => inst!(cpu, JP, (Condition::ALWAYS), nn),

		(3, 3, 1, _, _) => {
			let cb_opcode = Opcode::from(cpu.next_byte());
			match cb_opcode.x {
				0 => inst!(
					cpu,
					ROT,
					(DT.rot[cb_opcode.y as usize]),
					(DT.r[cb_opcode.z as usize])
				),
				1 => inst!(cpu, BIT, (cb_opcode.y), (DT.r[cb_opcode.z as usize])),
				2 => inst!(cpu, RES, (cb_opcode.y), (DT.r[cb_opcode.z as usize])),
				3 => inst!(cpu, SET, (cb_opcode.y), (DT.r[cb_opcode.z as usize])),
				_ => inst!(cpu, ERROR, (cb_opcode.raw)),
			}
		}

		(3, 3, 6, _, _) => inst!(cpu, DI),
		(3, 3, 7, _, _) => inst!(cpu, EI),

		(3, 4, 0, _, _) => inst!(cpu, CALL, (DT.cc[0]), nn),
		(3, 4, 1, _, _) => inst!(cpu, CALL, (DT.cc[1]), nn),
		(3, 4, 2, _, _) => inst!(cpu, CALL, (DT.cc[2]), nn),
		(3, 4, 3, _, _) => inst!(cpu, CALL, (DT.cc[3]), nn),

		(3, 5, _, 0, 0) => inst!(cpu, PUSH, (DT.rp2[0])),
		(3, 5, _, 1, 0) => inst!(cpu, PUSH, (DT.rp2[1])),
		(3, 5, _, 2, 0) => inst!(cpu, PUSH, (DT.rp2[2])),
		(3, 5, _, 3, 0) => inst!(cpu, PUSH, (DT.rp2[3])),

		(3, 5, _, 0, 1) => inst!(cpu, CALL, (Condition::ALWAYS), nn),

		(3, 6, _, _, _) => inst!(cpu, ALU_OP_8, (DT.alu[y]), A, n),
		(3, 7, _, _, _) => inst!(cpu, RST, ((opcode.y as u16) * 8)),
		(_, _, _, _, _) => inst!(cpu, ERROR, (opcode.raw)),
	}
}
