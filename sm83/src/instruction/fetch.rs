use super::{
	decode_tables::DT,
	opcode::{parse_opcode, Opcode},
	CPURegister16::*,
	CPURegister8::*,
	Condition, Instruction,
	Instruction::*,
	ValueRefU8,
};

use crate::{
	arg, inst, mem, registers::CPURegister16,
	values::ValueRefU16, SM83,
};

pub fn fetch(cpu: &mut impl SM83) -> Instruction {
	let raw = cpu.next_byte();

	let Opcode(x, z, y, p, q) = parse_opcode(raw);
	match (x, z, y, p, q) {
		(0, 0, 0, _, _) => NOP,
		(0, 0, 1, _, _) => inst!(cpu, LD_16, (ValueRefU16::Mem(cpu.next_chomp())), SP),
		(0, 0, 2, _, _) => inst!(cpu, STOP),

		(0, 0, 3, _, _) => inst!(cpu, JR, (Condition::Always), d),
		(0, 0, _, _, _) => inst!(cpu, JR, (DT.cc[y - 4]), d),

		(0, 1, _, 0, 0) => inst!(cpu, LD_16, BC, nn),
		(0, 1, _, 1, 0) => inst!(cpu, LD_16, DE, nn),
		(0, 1, _, 2, 0) => inst!(cpu, LD_16, HL, nn),
		(0, 1, _, 3, 0) => inst!(cpu, LD_16, SP, nn),

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

		(2, _, _, _, _) => inst!(cpu, ALU_OP_8, (DT.alu[y]), (DT.r[z])),

		(3, 0, 0, _, _) => inst!(cpu, RET, (Condition::NZ)),
		(3, 0, 1, _, _) => inst!(cpu, RET, (Condition::Z)),
		(3, 0, 2, _, _) => inst!(cpu, RET, (Condition::NC)),
		(3, 0, 3, _, _) => inst!(cpu, RET, (Condition::C)),

		(3, 0, 4, _, _) => inst!(cpu, LDH, (ValueRefU8::MemOffsetRaw(cpu.next_byte())), A),
		(3, 0, 5, _, _) => inst!(cpu, ADD_SIGNED, SP, d),
		(3, 0, 6, _, _) => inst!(cpu, LDH, A, (ValueRefU8::MemOffsetRaw(cpu.next_byte()))),

		(3, 0, 7, _, _) => inst!(cpu, LD_HL_SP_DD, d),

		(3, 1, _, 0, 0) => inst!(cpu, POP, BC),
		(3, 1, _, 1, 0) => inst!(cpu, POP, DE),
		(3, 1, _, 2, 0) => inst!(cpu, POP, HL),
		(3, 1, _, 3, 0) => inst!(cpu, POP, AF),

		(3, 1, _, 0, 1) => inst!(cpu, RET, (Condition::Always)),
		(3, 1, _, 1, 1) => inst!(cpu, RETI),
		(3, 1, _, 2, 1) => inst!(cpu, JP, (Condition::Always), HL),
		(3, 1, _, 3, 1) => inst!(cpu, LD_16, SP, HL),

		(3, 2, 4, _, _) => inst!(cpu, LDH, (ValueRefU8::MemOffsetReg(C)), A),
		(3, 2, 5, _, _) => inst!(cpu, LD_8, [nn]u8, A),
		(3, 2, 6, _, _) => inst!(cpu, LDH, A, (ValueRefU8::MemOffsetReg(C))),
		(3, 2, 7, _, _) => inst!(cpu, LD_8, A, [nn]u8),

		(3, 2, 0, _, _) => inst!(cpu, JP, (Condition::NZ), nn),
		(3, 2, 1, _, _) => inst!(cpu, JP, (Condition::Z), nn),
		(3, 2, 2, _, _) => inst!(cpu, JP, (Condition::NC), nn),
		(3, 2, 3, _, _) => inst!(cpu, JP, (Condition::C), nn),

		(3, 3, 0, _, _) => inst!(cpu, JP, (Condition::Always), nn),

		(3, 3, 1, _, _) => fetch_cb(cpu),

		(3, 3, 6, _, _) => inst!(cpu, DI),
		(3, 3, 7, _, _) => inst!(cpu, EI),

		(3, 4, 0, _, _) => inst!(cpu, CALL, (Condition::NZ), nn),
		(3, 4, 1, _, _) => inst!(cpu, CALL, (Condition::Z), nn),
		(3, 4, 2, _, _) => inst!(cpu, CALL, (Condition::NC), nn),
		(3, 4, 3, _, _) => inst!(cpu, CALL, (Condition::C), nn),

		(3, 5, _, 0, 0) => inst!(cpu, PUSH, (CPURegister16::BC)),
		(3, 5, _, 1, 0) => inst!(cpu, PUSH, (CPURegister16::DE)),
		(3, 5, _, 2, 0) => inst!(cpu, PUSH, (CPURegister16::HL)),
		(3, 5, _, 3, 0) => inst!(cpu, PUSH, (CPURegister16::AF)),

		(3, 5, _, 0, 1) => inst!(cpu, CALL, (Condition::Always), nn),

		(3, 6, y, _, _) => inst!(cpu, ALU_OP_8, (DT.alu[y]), n),
		(3, 7, y, _, _) => inst!(cpu, RST, ((y as u16) * 8)),
		(_, _, _, _, _) => inst!(cpu, ERROR, (raw)),
	}
}

#[inline]
fn fetch_cb(cpu: &mut impl SM83) -> Instruction {
	let raw = cpu.next_byte();
	let Opcode(x, z, y, _, _) = parse_opcode(raw);
	match (x, z, y) {
		(0, _, _) => inst!(cpu, ROT, (DT.rot[y]), (DT.r[z])),
		(1, _, _) => inst!(cpu, BIT, (y as u8), (DT.r[z])),
		(2, _, _) => inst!(cpu, RES, (y as u8), (DT.r[z])),
		(3, _, _) => inst!(cpu, SET, (y as u8), (DT.r[z])),
		(_, _, _) => inst!(cpu, ERROR, (raw)),
	}
}
