use crate::cpu::gbStack::GBStack;
use crate::cpu::registers::Register16;
use crate::cpu::values::ValueRefI8;
use crate::cpu::values::ValueRefU16;
use crate::cpu::values::ValueRefU8;

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
    JP(condition, location) => {
      if(cpu.check_condition(condition)) {
        cpu.write_16(
          ValueRefU16::Reg(Register16::PC), 
          cpu.read_16(location)
        );
      }
    },
    JR(condition, offset) => {
      if(cpu.check_condition(condition)) {
        let current_pc = cpu.read_16(ValueRefU16::Reg(Register16::PC));
        
        let offset = cpu.read_i8(offset);
            
        cpu.write_16(
          ValueRefU16::Reg(Register16::PC),
          (current_pc as i32 + offset as i32) as u16
        )
      }
    },
    ADD_16(_, _) => todo!(),
    ADD_SIGNED(_, _) => todo!(),
    ALU_OP_8(_, _, _) => todo!(),
    HALT => todo!(),
    CALL(condition, location) => {
      if(cpu.check_condition(condition)) {
        cpu.push(cpu.read_16(ValueRefU16::Reg(Register16::PC))+1);
        cpu.write_16(
          ValueRefU16::Reg(Register16::PC), 
          cpu.read_16(location)
        )
      }
    },
    POP(_) => todo!(),
    PUSH(_) => todo!(),
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