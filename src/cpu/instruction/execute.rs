use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;

use crate::cpu::flags::Flag;
use crate::cpu::flags::Flags;
use crate::cpu::gb_stack::GBStack;
use crate::cpu::registers::CPURegister16;
use crate::cpu::registers::CPURegister8;

use super::ALUOperation;
use super::Cpu;
use super::Instruction;
use super::Instruction::*;

pub fn execute_instruction(instruction: Instruction, cpu: &mut Cpu) {
	match instruction {
		NOP => {}

		COMPOSE(a, b) => {
			execute_instruction(*a, cpu);
			execute_instruction(*b, cpu);
		}

		LD_8(to, from) => {
			cpu.write_8(to, cpu.read_8(from));
		}

		LD_16(to, from) => {
			cpu.write_16(to, cpu.read_16(from));
		}

		INC_8(ptr) => {
			let val = cpu.read_8(ptr);
			cpu.set_flag_to(Flag::Z, val.wrapping_add(1) == 0);
			cpu.clear_flag(Flag::N);
			cpu.set_flag_to(Flag::H, ((val & 0xf).wrapping_add(1) & 0x10) == 0x10);
			cpu.write_8(ptr, val.wrapping_add(1));
		}

		INC_16(ptr) => {
			cpu.write_16(ptr, cpu.read_16(ptr) + 1);
		}

		DEC_8(ptr) => {
			let val = cpu.read_8(ptr);
			cpu.set_flag_to(Flag::Z, val.wrapping_sub(1) == 0);
			cpu.set_flag(Flag::N);
			cpu.set_flag_to(Flag::H, ((val & 0xf).wrapping_sub(1 & 0xf) & 0x10) == 0x10);
			cpu.write_8(ptr, val.wrapping_sub(1));
		}

		DEC_16(ptr) => {
			cpu.write_16(ptr, cpu.read_16(ptr) - 1);
		}

		STOP => todo!(),
		ERROR(opcode) => todo!(),

		JP(condition, location) => {
			if cpu.check_condition(condition) {
				cpu.write_16(CPURegister16::PC.into(), cpu.read_16(location));
			}
		}

		JR(condition, offset) => {
			if cpu.check_condition(condition) {
				let current_pc = cpu.read_16(CPURegister16::PC.into());

				let offset = cpu.read_i8(offset);

				cpu.write_16(
					CPURegister16::PC.into(),
					(current_pc as i32 + offset as i32) as u16,
				)
			}
		}

		ADD_16(a_ref, b_ref) => {
			let a_val = cpu.read_16(a_ref);
			let b_val = cpu.read_16(b_ref);
			cpu.clear_flag(Flag::N);
			cpu.set_flag(Flag::C);

			cpu.write_16(a_ref, a_val.wrapping_add(b_val));
		}

		ADD_SIGNED(_, _) => todo!(),
		ALU_OP_8(op, to, from) => {
			let a_val = cpu.read_8(to);
			let b_val = cpu.read_8(from);

			let carry: u8 = match cpu.get_flag(Flag::C) {
				false => 0,
				true => 1,
			};

			let result = match op {
				ALUOperation::ADD => {
					cpu.clear_flag(Flag::N);
					cpu.set_flag_to(Flag::H, ((a_val & 0xf).wrapping_add(b_val) & 0x10) == 0x10);
					cpu.set_flag_to(Flag::C, a_val.wrapping_add(b_val) < a_val);
					cpu.set_flag_to(Flag::Z, a_val.wrapping_add(b_val) == 0);
					a_val.wrapping_add(b_val)
				}
				ALUOperation::ADC => {
					cpu.clear_flag(Flag::N);
					cpu.set_flag_to(
						Flag::H,
						((a_val & 0xf).wrapping_add(b_val).wrapping_add(carry) & 0x10) == 0x10,
					);
					cpu.set_flag_to(
						Flag::C,
						a_val.wrapping_add(b_val).wrapping_add(carry) < a_val,
					);
					cpu.set_flag_to(Flag::Z, a_val.wrapping_add(b_val).wrapping_add(carry) == 0);
					a_val.wrapping_add(b_val).wrapping_add(carry)
				}
				ALUOperation::SUB => {
					cpu.set_flag(Flag::N);
					cpu.set_flag_to(Flag::H, ((a_val & 0xf).wrapping_sub(b_val) & 0x10) == 0x10);
					cpu.set_flag_to(Flag::C, a_val.wrapping_sub(b_val) > a_val);
					cpu.set_flag_to(Flag::Z, a_val.wrapping_sub(b_val) == 0);
					a_val.wrapping_sub(b_val)
				}
				ALUOperation::SBC => {
					cpu.set_flag(Flag::N);
					cpu.set_flag_to(
						Flag::H,
						((a_val & 0xf).wrapping_sub(b_val).wrapping_sub(carry) & 0x10) == 0x10,
					);
					cpu.set_flag_to(
						Flag::C,
						a_val.wrapping_sub(b_val).wrapping_sub(carry) > a_val,
					);
					cpu.set_flag_to(Flag::Z, a_val.wrapping_sub(b_val).wrapping_sub(carry) == 0);
					a_val.wrapping_sub(b_val).wrapping_sub(carry)
				}
				ALUOperation::AND => a_val.bitand(b_val),
				ALUOperation::XOR => a_val.bitxor(b_val),
				ALUOperation::OR => a_val.bitor(b_val),
				ALUOperation::CP => {
					cpu.set_flag(Flag::N);
					cpu.set_flag_to(Flag::H, ((a_val & 0xf).wrapping_sub(b_val) & 0x10) == 0x10);
					cpu.set_flag_to(Flag::C, a_val.wrapping_sub(b_val) > a_val);
					cpu.set_flag_to(Flag::Z, a_val.wrapping_sub(b_val) == 0);
					a_val
				}
			};

			if result == 0 {
				cpu.set_flag(Flag::Z)
			};

			cpu.write_8(to, result);
		}
		HALT => todo!(),
		CALL(condition, location) => {
			if cpu.check_condition(condition) {
				cpu.push(cpu.read_16(CPURegister16::PC.into()));

				cpu.write_16(CPURegister16::PC.into(), cpu.read_16(location));
			}
		}
		POP(value_ref) => {
			let val = cpu.pop();
			cpu.write_16(value_ref.into(), val);
		}
		PUSH(value_ref) => cpu.push(cpu.read_16(value_ref.into())),
		RET(condition) => {
			if cpu.check_condition(condition) {
				let ptr = cpu.pop();
				cpu.write_16(CPURegister16::PC.into(), ptr);
			}
		}
		RETI => todo!(),
		RST(addr) => {
			cpu.push(cpu.read_16(CPURegister16::PC.into()));
			cpu.write_16(CPURegister16::PC.into(), cpu.read_16(addr));
		}
		DI => todo!(),
		EI => todo!(),
		RLCA => {
			let value = cpu.read_8(CPURegister8::A.into());
			if value & 1 != 0 {
				cpu.set_flag(Flag::C)
			}
			cpu.write_8(CPURegister8::A.into(), value.rotate_left(1));
		}
		RRCA => {
			let value = cpu.read_8(CPURegister8::A.into());
			if value.rotate_right(1) & 1 != 0 {
				cpu.set_flag(Flag::C)
			}
			cpu.write_8(CPURegister8::A.into(), value.rotate_right(1));
		}

		RLA => execute_instruction(
			Instruction::ROT(super::RotShiftOperation::RL, CPURegister8::A.into()),
			cpu,
		),
		RRA => execute_instruction(
			Instruction::ROT(super::RotShiftOperation::RR, CPURegister8::A.into()),
			cpu,
		),

		DAA => todo!(),
		CPL => todo!(),
		SCF => todo!(),
		CCF => todo!(),
		BIT(bit, value) => {
			cpu.set_flag_to(Flag::Z, (cpu.read_8(value.into()) >> bit) & 1 == 0);
			cpu.set_flag(Flag::H);
			cpu.clear_flag(Flag::N);
		}
		RES(_, _) => todo!(),
		SET(_, _) => todo!(),
		ROT(operator, val_ref) => {
			use super::RotShiftOperation::*;
			let value = cpu.read_8(val_ref);

			let carry_bit = match cpu.get_flag(Flag::C) {
				true => 1,
				false => 0,
			};

			match operator {
				RLC => {
					if value & 1 != 0 {
						cpu.set_flag(Flag::C)
					}
					cpu.write_8(val_ref, value.rotate_left(1));
				}
				RRC => {
					if value.rotate_right(1) & 1 != 0 {
						cpu.set_flag(Flag::C)
					}
					cpu.write_8(val_ref, value.rotate_right(1));
				}
				RL => cpu.write_8(val_ref, ((value << 1) & 0b11111110) | carry_bit),
				RR => cpu.write_8(val_ref, ((value >> 1) & 0b01111111) | (carry_bit << 7)),
				SLA => {
					if (value >> 7) & 1 != 0 {
						cpu.set_flag(Flag::C)
					}
					cpu.write_8(val_ref, value << 1);
				}
				SRA => {
					if value & 1 != 0 {
						cpu.set_flag(Flag::C)
					}
					cpu.write_8(val_ref, (value >> 1) | value & (1 << 7));
				}
				SWAP => cpu.write_8(val_ref, value.rotate_right(4)),
				SRL => {
					if value & 1 != 0 {
						cpu.set_flag(Flag::C)
					}
					cpu.write_8(val_ref, value >> 1);
				}
			}
		}
	}
}
