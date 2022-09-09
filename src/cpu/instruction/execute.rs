use super::Cpu;
use super::Instruction;
use super::Instruction::*;

pub fn execute_instruction(instruction:Instruction, cpu:&mut Cpu) {
	match instruction {
		NOP => {},

    LD_8(to, from) => {
			cpu.write_8(to, cpu.read_8(from));
		},
    LDD_8(to, from) => {
			cpu.write_8(to, cpu.read_8(from));
			cpu.write_8(from, cpu.read_8(from) - 1);
    },
    LDI_8(to, from) => {
			cpu.write_8(to, cpu.read_8(from));
			cpu.write_8(from, cpu.read_8(from) + 1);
    },

    LD_16(to, from) => {
			cpu.write_16(to, cpu.read_16(from));
		},

    INC_8(ptr) => {
			cpu.write_8(ptr, cpu.read_8(ptr) + 1);
		},
    INC_16(ptr) => {
			cpu.write_16(ptr, cpu.read_16(ptr) + 1);
		},

    DEC_8(ptr) => {
			cpu.write_8(ptr, cpu.read_8(ptr) - 1);
		},
    DEC_16(ptr) => {
			cpu.write_16(ptr, cpu.read_16(ptr) - 1);
		},

    STOP => todo!(),
    ERROR => todo!(),
    JR(_, _) => todo!(),
    ADD_16(_, _) => todo!(),
    ADD_SIGNED(_, _) => todo!(),
    ALU_OP_8(_, _, _) => todo!(),
    HALT => todo!(),
    CALL(_, _) => todo!(),
    POP(_) => todo!(),
    PUSH(_) => todo!(),
    JP(_, _) => todo!(),
    RET(_) => todo!(),
    RETI => todo!(),
    RST(_) => todo!(),
    DI => todo!(),
    EI => todo!(),
    RLCA => todo!(),
    RRCA => todo!(),
    RLA => todo!(),
    RRA => todo!(),
    DAA => todo!(),
    CPL => todo!(),
    SCF => todo!(),
    CCF => todo!(),
    BIT(_, _) => todo!(),
    RES(_, _) => todo!(),
    SET(_, _) => todo!(),
    ROT(_, _) => todo!(),
	}
}