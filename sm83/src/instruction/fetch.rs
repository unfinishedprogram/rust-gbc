use super::{
	decode_tables::DT,
	opcode::{parse_opcode, Opcode},
	CPURegister16::*,
	CPURegister8::*,
	Condition, Instruction,
	Instruction::*,
	ValueRefU8,
};

use crate::{arg, inst, mem, registers::CPURegister16, values::ValueRefU16, SM83};

pub trait Fetch {
	fn fetch(&mut self) -> Instruction;
	fn fetch_cb(&mut self) -> Instruction;
}

impl<T: SM83> Fetch for T {
	fn fetch(&mut self) -> Instruction {
		let raw = self.next_byte();

		let Opcode(x, z, y, p, q) = parse_opcode(raw);
		match (x, z, y, p, q) {
			(0, 0, 0, _, _) => NOP,
			(0, 0, 1, _, _) => inst!(self, LD_16, (ValueRefU16::Mem(self.next_chomp())), SP),
			(0, 0, 2, _, _) => inst!(self, STOP),

			(0, 0, 3, _, _) => inst!(self, JR, (Condition::Always), d),
			(0, 0, _, _, _) => inst!(self, JR, (DT.cc[y - 4]), d),

			(0, 1, _, 0, 0) => inst!(self, LD_16, BC, nn),
			(0, 1, _, 1, 0) => inst!(self, LD_16, DE, nn),
			(0, 1, _, 2, 0) => inst!(self, LD_16, HL, nn),
			(0, 1, _, 3, 0) => inst!(self, LD_16, SP, nn),

			(0, 1, _, _, 1) => inst!(self, ADD_16, HL, (DT.rp[p])),

			(0, 2, _, 0, 0) => inst!(self, LD_8, [BC]u8, A),
			(0, 2, _, 1, 0) => inst!(self, LD_8, [DE]u8, A),

			(0, 2, _, 2, 0) => inst!(self, LD_INC_HL_A),

			(0, 2, _, 3, 0) => inst!(self, LD_DEC_HL_A),

			(0, 2, _, 0, 1) => inst!(self, LD_8, A, [BC]u8),
			(0, 2, _, 1, 1) => inst!(self, LD_8, A, [DE]u8),

			(0, 2, _, 2, 1) => inst!(self, LD_A_INC_HL),
			(0, 2, _, 3, 1) => inst!(self, LD_A_DEC_HL),

			(0, 3, _, _, 0) => inst!(self, INC_16, (DT.rp[p])),
			(0, 3, _, _, 1) => inst!(self, DEC_16, (DT.rp[p])),
			(0, 4, _, _, _) => inst!(self, INC_8, (DT.r[y])),
			(0, 5, _, _, _) => inst!(self, DEC_8, (DT.r[y])),
			(0, 6, _, _, _) => inst!(self, LD_8, (DT.r[y]), n),

			(0, 7, 0, _, _) => inst!(self, RLCA),
			(0, 7, 1, _, _) => inst!(self, RRCA),
			(0, 7, 2, _, _) => inst!(self, RLA),
			(0, 7, 3, _, _) => inst!(self, RRA),
			(0, 7, 4, _, _) => inst!(self, DAA),
			(0, 7, 5, _, _) => inst!(self, CPL),
			(0, 7, 6, _, _) => inst!(self, SCF),
			(0, 7, 7, _, _) => inst!(self, CCF),

			(1, 6, 6, _, _) => inst!(self, HALT),
			(1, _, _, _, _) => inst!(self, LD_8, (DT.r[y]), (DT.r[z])),

			(2, _, _, _, _) => inst!(self, ALU_OP_8, (DT.alu[y]), (DT.r[z])),

			(3, 0, 0, _, _) => inst!(self, RET, (Condition::NZ)),
			(3, 0, 1, _, _) => inst!(self, RET, (Condition::Z)),
			(3, 0, 2, _, _) => inst!(self, RET, (Condition::NC)),
			(3, 0, 3, _, _) => inst!(self, RET, (Condition::C)),

			(3, 0, 4, _, _) => inst!(self, LDH, (ValueRefU8::MemOffsetRaw(self.next_byte())), A),
			(3, 0, 5, _, _) => inst!(self, ADD_SIGNED, SP, d),
			(3, 0, 6, _, _) => inst!(self, LDH, A, (ValueRefU8::MemOffsetRaw(self.next_byte()))),

			(3, 0, 7, _, _) => inst!(self, LD_HL_SP_DD, d),

			(3, 1, _, 0, 0) => inst!(self, POP, BC),
			(3, 1, _, 1, 0) => inst!(self, POP, DE),
			(3, 1, _, 2, 0) => inst!(self, POP, HL),
			(3, 1, _, 3, 0) => inst!(self, POP, AF),

			(3, 1, _, 0, 1) => inst!(self, RET, (Condition::Always)),
			(3, 1, _, 1, 1) => inst!(self, RETI),
			(3, 1, _, 2, 1) => inst!(self, JP, (Condition::Always), HL),
			(3, 1, _, 3, 1) => inst!(self, LD_16, SP, HL),

			(3, 2, 4, _, _) => inst!(self, LDH, (ValueRefU8::MemOffsetReg(C)), A),
			(3, 2, 5, _, _) => inst!(self, LD_8, [nn]u8, A),
			(3, 2, 6, _, _) => inst!(self, LDH, A, (ValueRefU8::MemOffsetReg(C))),
			(3, 2, 7, _, _) => inst!(self, LD_8, A, [nn]u8),

			(3, 2, 0, _, _) => inst!(self, JP, (Condition::NZ), nn),
			(3, 2, 1, _, _) => inst!(self, JP, (Condition::Z), nn),
			(3, 2, 2, _, _) => inst!(self, JP, (Condition::NC), nn),
			(3, 2, 3, _, _) => inst!(self, JP, (Condition::C), nn),

			(3, 3, 0, _, _) => inst!(self, JP, (Condition::Always), nn),

			(3, 3, 1, _, _) => self.fetch_cb(),

			(3, 3, 6, _, _) => inst!(self, DI),
			(3, 3, 7, _, _) => inst!(self, EI),

			(3, 4, 0, _, _) => inst!(self, CALL, (Condition::NZ), nn),
			(3, 4, 1, _, _) => inst!(self, CALL, (Condition::Z), nn),
			(3, 4, 2, _, _) => inst!(self, CALL, (Condition::NC), nn),
			(3, 4, 3, _, _) => inst!(self, CALL, (Condition::C), nn),

			(3, 5, _, 0, 0) => inst!(self, PUSH, (CPURegister16::BC)),
			(3, 5, _, 1, 0) => inst!(self, PUSH, (CPURegister16::DE)),
			(3, 5, _, 2, 0) => inst!(self, PUSH, (CPURegister16::HL)),
			(3, 5, _, 3, 0) => inst!(self, PUSH, (CPURegister16::AF)),

			(3, 5, _, 0, 1) => inst!(self, CALL, (Condition::Always), nn),

			(3, 6, y, _, _) => inst!(self, ALU_OP_8, (DT.alu[y]), n),
			(3, 7, y, _, _) => inst!(self, RST, ((y as u16) * 8)),
			(_, _, _, _, _) => inst!(self, ERROR, (raw)),
		}
	}

	fn fetch_cb(&mut self) -> Instruction {
		let raw = self.next_byte();
		let Opcode(x, z, y, _, _) = parse_opcode(raw);
		match (x, z, y) {
			(0, _, _) => inst!(self, ROT, (DT.rot[y]), (DT.r[z])),
			(1, _, _) => inst!(self, BIT, (y as u8), (DT.r[z])),
			(2, _, _) => inst!(self, RES, (y as u8), (DT.r[z])),
			(3, _, _) => inst!(self, SET, (y as u8), (DT.r[z])),
			(_, _, _) => inst!(self, ERROR, (raw)),
		}
	}
}
