use std::ops::BitAnd;
use std::ops::BitXor;

use crate::cpu::flags::Flag;
use crate::cpu::flags::Flags;
use crate::cpu::gb_stack::GBStack;
use crate::cpu::registers::CPURegister16;
use crate::console_log;
use crate::log;

use super::Cpu;
use super::Instruction;
use super::Instruction::*;
use super::ALUOperation;

pub fn execute_instruction(instruction:Instruction, cpu:&mut Cpu) {
  console_log!("{:?}", instruction);
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
    ERROR(opcode) => todo!(),
    JP(condition, location) => {
      if cpu.check_condition(condition) {
        cpu.write_16(
          CPURegister16::PC.into(), 
          cpu.read_16(location)
        );
      }
    },
    JR(condition, offset) => {
      if cpu.check_condition(condition) {
        let current_pc = cpu.read_16(CPURegister16::PC.into());
        
        let offset = cpu.read_i8(offset);
            
        cpu.write_16(
          CPURegister16::PC.into(),
          (current_pc as i32 + offset as i32) as u16
        )
      }
    },
    ADD_16(_, _) => todo!(),
    ADD_SIGNED(_, _) => todo!(),
    ALU_OP_8(op, to, from) => match op {
        ALUOperation::ADD => todo!(),
        ALUOperation::ADC => todo!(),
        ALUOperation::SUB => todo!(),
        ALUOperation::SBC => todo!(),
        ALUOperation::AND => todo!(),
        ALUOperation::XOR => cpu.write_8(to, cpu.read_8(from).bitxor(cpu.read_8(to))),
        ALUOperation::OR => todo!(),
        ALUOperation::CP => todo!(),
    },
    HALT => todo!(),
    CALL(condition, location) => {
      if cpu.check_condition(condition) {
        cpu.push(
          cpu.read_16(CPURegister16::PC.into())+1
        );

        cpu.write_16(
          CPURegister16::PC.into(), 
          cpu.read_16(location)
        );
      }
    },
    POP(_) => todo!(),
    PUSH(_) => todo!(),
    RET(condition) => {
      if cpu.check_condition(condition) {
        let ptr = cpu.pop();
        cpu.write_16(CPURegister16::PC.into(), ptr);
      }
    },
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
    BIT(bit, value) => {
      cpu.set_flag_to(Flag::Z,
        (cpu.read_8(value.into()) >> bit) & 1 != 0
      )
    },
    RES(_, _) => todo!(),
    SET(_, _) => todo!(),
    ROT(_, _) => todo!(),
	}
}