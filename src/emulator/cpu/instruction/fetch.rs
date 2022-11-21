use super::opcode::parse_opcode;
use super::{
	decode_tables::DT, CPURegister16::*, CPURegister8::*, Condition, Instruction, Instruction::*,
	ValueRefU8,
};

use crate::emulator::cpu::registers::CPURegister16;
use crate::emulator::cpu::values::ValueRefU16;
use crate::emulator::cpu::CPU;
use crate::emulator::memory_mapper::MemoryMapper;

use crate::{arg, inst, mem}; // Macros

pub fn fetch_instruction<T: CPU + MemoryMapper>(cpu: &mut T) -> Instruction {
	let raw = cpu.next_byte();

	let (x, z, y, p, q) = parse_opcode(raw);
	match (x, z, y, p, q) {
		//(x, z, y, p, q)
		(0, 0, 0, _, _) => inst!(cpu, NOP),
		(0, 0, 1, _, _) => inst!(cpu, LD_16, (ValueRefU16::Mem(cpu.next_chomp())), SP),
		(0, 0, 2, _, _) => inst!(cpu, STOP),
		(0, 0, 3, _, _) => {
			let offset = cpu.next_displacement();
			let addr = if offset.is_positive() {
				cpu.read_16(CPURegister16::PC.into()) + offset.unsigned_abs() as u16
			} else {
				cpu.read_16(CPURegister16::PC.into()) - offset.unsigned_abs() as u16
			};

			inst!(cpu, JR, (Condition::ALWAYS), (ValueRefU16::Raw(addr)))
		}

		(0, 0, _, _, _) => {
			let offset = cpu.next_displacement();
			let addr = if offset.is_positive() {
				cpu.read_16(CPURegister16::PC.into()) + offset.unsigned_abs() as u16
			} else {
				cpu.read_16(CPURegister16::PC.into()) - offset.unsigned_abs() as u16
			};

			inst!(cpu, JR, (DT.cc[(y - 4)]), (ValueRefU16::Raw(addr)))
		}

		(0, 1, _, _, 0) => inst!(cpu, LD_16, (DT.rp[p]), nn),

		(0, 1, _, _, 1) => inst!(cpu, ADD_16, HL, (DT.rp[p])),

		(0, 2, _, 0, 0) => inst!(cpu, LD_8, [BC]u8, A),
		(0, 2, _, 1, 0) => inst!(cpu, LD_8, [DE]u8, A),

		(0, 2, _, 2, 0) => inst!(cpu, LD_INC_HL_A),

		(0, 2, _, 3, 0) => inst!(cpu, LD_DEC_HL_A),

		(0, 2, _, 0, 1) => inst!(cpu, LD_8, A, [BC]u8),
		(0, 2, _, 1, 1) => inst!(cpu, LD_8, A, [DE]u8),

		(0, 2, _, 2, 1) => inst!(cpu, LD_A_INC_HL),
		(0, 2, _, 3, 1) => inst!(cpu, LD_A_DEC_HL),

		(0, 3, _, _, 0) => inst!(cpu, INC_16, (DT.rp[p])),
		(0, 3, _, _, 1) => inst!(cpu, DEC_16, (DT.rp[p])),
		(0, 4, _, _, _) => inst!(cpu, INC_8, (DT.r[y].clone())),
		(0, 5, _, _, _) => inst!(cpu, DEC_8, (DT.r[y].clone())),
		(0, 6, _, _, _) => inst!(cpu, LD_8, (DT.r[y].clone()), n),

		(0, 7, 0, _, _) => inst!(cpu, RLCA),
		(0, 7, 1, _, _) => inst!(cpu, RRCA),
		(0, 7, 2, _, _) => inst!(cpu, RLA),
		(0, 7, 3, _, _) => inst!(cpu, RRA),
		(0, 7, 4, _, _) => inst!(cpu, DAA),
		(0, 7, 5, _, _) => inst!(cpu, CPL),
		(0, 7, 6, _, _) => inst!(cpu, SCF),
		(0, 7, 7, _, _) => inst!(cpu, CCF),

		(1, 6, 6, _, _) => inst!(cpu, HALT),
		(1, _, _, _, _) => inst!(cpu, LD_8, (DT.r[y].clone()), (DT.r[z].clone())),

		(2, _, _, _, _) => inst!(cpu, ALU_OP_8, (DT.alu[y]), A, (DT.r[z].clone())),

		(3, 0, 0, _, _) => inst!(cpu, RET, (DT.cc[0])),
		(3, 0, 1, _, _) => inst!(cpu, RET, (DT.cc[1])),
		(3, 0, 2, _, _) => inst!(cpu, RET, (DT.cc[2])),
		(3, 0, 3, _, _) => inst!(cpu, RET, (DT.cc[3])),

		(3, 0, 4, _, _) => {
			inst!(
				cpu,
				LDH,
				(ValueRefU8::MemOffset(Box::new(ValueRefU8::Raw(cpu.next_byte())))),
				A
			)
		}

		(3, 0, 5, _, _) => inst!(cpu, ADD_SIGNED, SP, d),

		(3, 0, 6, _, _) => {
			inst!(
				cpu,
				LDH,
				A,
				(ValueRefU8::MemOffset(Box::new(ValueRefU8::Raw(cpu.next_byte()))))
			)
		}

		(3, 0, 7, _, _) => inst!(cpu, LD_HL_SP_DD, d),

		(3, 1, _, _, 0) => inst!(cpu, POP, (DT.rp2[p])),

		(3, 1, _, 0, 1) => inst!(cpu, RET, (Condition::ALWAYS)),
		(3, 1, _, 1, 1) => inst!(cpu, RETI),
		(3, 1, _, 2, 1) => inst!(cpu, JP, (Condition::ALWAYS), HL),
		(3, 1, _, 3, 1) => inst!(cpu, LD_16, SP, HL),

		(3, 2, 4, _, _) => {
			inst!(cpu, LDH, (ValueRefU8::MemOffset(Box::new(C.into()))), A)
		}
		(3, 2, 5, _, _) => inst!(cpu, LD_8, [nn]u8, A),

		(3, 2, 6, _, _) => inst!(cpu, LD_8, A, (ValueRefU8::MemOffset(Box::new(C.into())))),
		(3, 2, 7, _, _) => inst!(cpu, LD_8, A, [nn]u8),

		(3, 2, c, _, _) => inst!(cpu, JP, (DT.cc[c]), nn),

		(3, 3, 0, _, _) => inst!(cpu, JP, (Condition::ALWAYS), nn),

		(3, 3, 1, _, _) => {
			let cb_raw = cpu.next_byte();
			let (cb_x, cb_z, cb_y, _, _) = parse_opcode(cb_raw);
			match cb_x {
				0 => inst!(cpu, ROT, (DT.rot[cb_y]), (DT.r[cb_z].clone())),
				1 => inst!(cpu, BIT, (cb_y as u8), (DT.r[cb_z].clone())),
				2 => inst!(cpu, RES, (cb_y as u8), (DT.r[cb_z].clone())),
				3 => inst!(cpu, SET, (cb_y as u8), (DT.r[cb_z].clone())),
				_ => inst!(cpu, ERROR, (cb_raw)),
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
		(3, 7, _, _, _) => inst!(cpu, RST, ((y as u16) * 8)),
		(_, _, _, _, _) => inst!(cpu, ERROR, (raw)),
	}
}
