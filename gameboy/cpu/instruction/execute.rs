use crate::{
	cpu::{
		flags::{Flag, Flags},
		gb_stack::GBStack,
		instruction::{ALUOperation, Instruction, Instruction::*},
		registers::{CPURegister16, CPURegister8},
		values::ValueRefU16,
		CPU,
	},
	flags::{INT_JOY_PAD, INT_LCD_STAT, INT_SERIAL, INT_TIMER, INT_V_BLANK},
	Gameboy,
};

use std::ops::{BitAnd, BitOr, BitXor};

use super::condition::Condition;

pub fn execute_instruction(instruction: Instruction, state: &mut Gameboy) {
	let cpu = state;

	match instruction {
		NOP => {}
		INT(interrupt) => {
			cpu.tick_m_cycles(3);

			cpu.disable_interrupts();
			let location = match interrupt {
				_ if interrupt == INT_V_BLANK => 0x40,
				_ if interrupt == INT_LCD_STAT => 0x48,
				_ if interrupt == INT_TIMER => 0x50,
				_ if interrupt == INT_SERIAL => 0x58,
				_ if interrupt == INT_JOY_PAD => 0x60,
				_ => unreachable!(),
			};
			let current_pc = cpu.read_16(&CPURegister16::PC.into());
			cpu.push(current_pc);
			cpu.write_16(&CPURegister16::PC.into(), location);
			cpu.disable_interrupts();
		}

		COMPOSE(a, b) => {
			execute_instruction(*a, cpu);
			execute_instruction(*b, cpu);
		}

		LDH(to, from) => {
			let val = cpu.read_8(&from);
			cpu.write_8(&to, val);
		}

		LD_8(to, from) => {
			let val = cpu.read_8(&from);
			cpu.write_8(&to, val);
		}

		LD_16(to, from) => {
			use ValueRefU16::*;
			if matches!((&to, &from), (Reg(_), Reg(_))) {
				cpu.tick_m_cycles(1);
			}

			let val = cpu.read_16(&from);
			cpu.write_16(&to, val);
		}

		INC_8(ptr) => {
			let val = cpu.read_8(&ptr);
			cpu.set_flag_to(Flag::Z, val.wrapping_add(1) == 0);
			cpu.clear_flag(Flag::N);
			cpu.set_flag_to(Flag::H, ((val & 0xf).wrapping_add(1) & 0x10) == 0x10);
			cpu.write_8(&ptr, val.wrapping_add(1));
		}

		INC_16(ptr) => {
			cpu.tick_m_cycles(1);
			let ptr_val = cpu.read_16(&ptr);
			cpu.write_16(&ptr, ptr_val.wrapping_add(1));
		}

		DEC_8(ptr) => {
			let val = cpu.read_8(&ptr);
			cpu.set_flag_to(Flag::Z, val.wrapping_sub(1) == 0);
			cpu.set_flag(Flag::N);
			cpu.set_flag_to(Flag::H, ((val & 0xf).wrapping_sub(1 & 0xf) & 0x10) == 0x10);
			cpu.write_8(&ptr, val.wrapping_sub(1));
		}

		DEC_16(ptr) => {
			cpu.tick_m_cycles(1);

			let ptr_val = cpu.read_16(&ptr);

			match ptr {
				ValueRefU16::Reg(_) => {}
				_ => {
					cpu.set_flag(Flag::N);
					cpu.set_flag_to(Flag::Z, ptr_val.wrapping_sub(1) == 0);
					cpu.set_flag_to(Flag::H, (((ptr_val & 0xf) - 1) & 0x10) == 0x10);
				}
			}

			cpu.write_16(&ptr, ptr_val.wrapping_sub(1));
		}

		STOP => println!("STOP"),
		ERROR(_) => {}

		JP(condition, location) | JR(condition, location) => {
			if cpu.check_condition(condition) {
				if !matches!(location, ValueRefU16::Reg(CPURegister16::HL)) {
					cpu.tick_m_cycles(1);
				}

				let loc_val = cpu.read_16(&location);
				cpu.write_16(&CPURegister16::PC.into(), loc_val);
			}
		}

		ADD_16(a_ref, b_ref) => {
			cpu.tick_m_cycles(1);

			let a_val = cpu.read_16(&a_ref);
			let b_val = cpu.read_16(&b_ref);

			cpu.set_flag_to(
				Flag::H,
				(((a_val & 0xFFF) + (b_val & 0xFFF)) & 0x1000) == 0x1000,
			);

			cpu.clear_flag(Flag::N);
			cpu.set_flag_to(Flag::C, a_val.wrapping_add(b_val) < a_val);
			cpu.write_16(&a_ref, a_val.wrapping_add(b_val));
		}

		ADD_SIGNED(a_ref, b_ref) => {
			cpu.tick_m_cycles(2);

			cpu.clear_flag(Flag::Z);
			cpu.clear_flag(Flag::N);

			let a_val = cpu.read_16(&a_ref);
			let b_val = cpu.read_i8(&b_ref) as i16;

			let ub_val = if b_val < 0 {
				((b_val as u16) ^ 0xFFFF).wrapping_sub(1)
			} else {
				b_val as u16
			};

			cpu.set_flag_to(Flag::C, (a_val << 8).wrapping_add(ub_val << 8) < a_val << 8);

			cpu.set_flag_to(
				Flag::H,
				((a_val & 0xf).wrapping_add(ub_val & 0xf) & 0x10) == 0x10,
			);

			cpu.write_16(&a_ref, a_val.wrapping_add_signed(b_val));
		}

		ALU_OP_8(op, to, from) => {
			use Flag::*;
			let a_val = cpu.read_8(&to);
			let b_val = cpu.read_8(&from);

			let carry = u8::from(cpu.get_flag(Flag::C));

			let result = match op {
				ALUOperation::ADD => {
					cpu.clear_flag(N);
					cpu.set_flag_to(H, ((a_val & 0xf).wrapping_add(b_val & 0xf) & 0x10) == 0x10);
					cpu.set_flag_to(C, a_val.wrapping_add(b_val) < a_val);
					a_val.wrapping_add(b_val)
				}
				ALUOperation::ADC => {
					let sum: u16 = a_val as u16 + b_val as u16 + carry as u16;
					cpu.set_flag_to(
						H,
						(a_val & 0xF).wrapping_add(b_val & 0xF).wrapping_add(carry) > 0xF,
					);
					cpu.set_flag_to(C, sum > 0xFF);
					cpu.clear_flag(N);

					sum as u8
				}

				ALUOperation::SUB => {
					cpu.set_flag(N);
					cpu.set_flag_to(H, (a_val & 0xF).wrapping_sub(b_val & 0xF) & 0x10 == 0x10);
					cpu.set_flag_to(C, b_val > a_val);
					a_val.wrapping_sub(b_val)
				}

				ALUOperation::SBC => {
					let sum: i32 = a_val as i32 - b_val as i32 - carry as i32;
					cpu.set_flag(N);
					cpu.set_flag_to(
						H,
						(a_val & 0xF) as i32 - (b_val & 0xF) as i32 - (carry as i32) < 0,
					);
					cpu.set_flag_to(C, sum < 0);
					(sum & 0xFF) as u8
				}
				ALUOperation::AND => {
					cpu.clear_flag(C);
					cpu.set_flag(H);
					cpu.clear_flag(N);
					a_val.bitand(b_val)
				}
				ALUOperation::XOR => {
					cpu.clear_flag(C);
					cpu.clear_flag(H);
					cpu.clear_flag(N);
					a_val.bitxor(b_val)
				}
				ALUOperation::OR => {
					cpu.clear_flag(C);
					cpu.clear_flag(H);
					cpu.clear_flag(N);
					a_val.bitor(b_val)
				}

				ALUOperation::CP => {
					cpu.set_flag_to(N, true);
					cpu.set_flag_to(C, a_val < b_val);
					cpu.set_flag_to(Z, a_val == b_val);
					cpu.set_flag_to(H, (a_val & 0xf).wrapping_sub(b_val & 0xf) & 0x10 == 0x10);
					a_val
				}
			};

			match op {
				ALUOperation::CP => {}
				_ => {
					cpu.write_8(&to, result);
					cpu.set_flag_to(Z, result == 0);
				}
			}
		}
		HALT => cpu.halted = true,
		CALL(condition, location) => {
			if cpu.check_condition(condition) {
				cpu.tick_m_cycles(1);
				let current_pc = cpu.read_16(&CPURegister16::PC.into());
				cpu.push(current_pc);
				let loc_value = cpu.read_16(&location);
				cpu.write_16(&CPURegister16::PC.into(), loc_value);
			}
		}
		POP(value_ref) => {
			cpu.tick_m_cycles(2);
			let val = cpu.pop();
			cpu.write_16(&value_ref.into(), val);
		}
		PUSH(value_ref) => {
			let value = cpu.read_16(&value_ref.into());
			cpu.tick_m_cycles(1);
			cpu.push(value)
		}
		RET(condition) => {
			use super::condition::Condition::*;

			if !matches!(condition, ALWAYS) {
				cpu.tick_m_cycles(1);
			}

			if matches!(condition, Condition::ALWAYS) || cpu.check_condition(condition) {
				cpu.tick_m_cycles(3);
				let ptr = cpu.pop();
				cpu.write_16(&CPURegister16::PC.into(), ptr);
			}
		}
		RST(addr) => {
			cpu.tick_m_cycles(1);
			let current_pc = cpu.read_16(&CPURegister16::PC.into());
			cpu.push(current_pc);
			let new_pc = cpu.read_16(&addr);
			cpu.write_16(&CPURegister16::PC.into(), new_pc);
		}
		DI => {
			cpu.disable_interrupts();
		}
		EI => {
			cpu.enable_interrupts();
		}
		RLCA => {
			let value = cpu.read_8(&CPURegister8::A.into());
			cpu.clear_flag(Flag::N);
			cpu.clear_flag(Flag::H);
			cpu.clear_flag(Flag::Z);
			cpu.set_flag_to(Flag::C, value & 0b10000000 == 0b10000000);
			cpu.write_8(&CPURegister8::A.into(), value.rotate_left(1));
		}
		RRCA => {
			let value = cpu.read_8(&CPURegister8::A.into());
			cpu.clear_flag(Flag::N);
			cpu.clear_flag(Flag::H);
			cpu.set_flag_to(Flag::C, value & 1 == 1);
			cpu.clear_flag(Flag::Z);
			cpu.write_8(&CPURegister8::A.into(), value.rotate_right(1));
		}

		RLA => {
			execute_instruction(
				Instruction::ROT(super::RotShiftOperation::RL, CPURegister8::A.into()),
				cpu,
			);
			cpu.clear_flag(Flag::Z);
		}
		RRA => {
			execute_instruction(
				Instruction::ROT(super::RotShiftOperation::RR, CPURegister8::A.into()),
				cpu,
			);
			cpu.clear_flag(Flag::Z);
			cpu.clear_flag(Flag::H);
			cpu.clear_flag(Flag::N);
		}
		DAA => {
			// Decimal Adjust A Register
			let a_ref = &CPURegister8::A.into();
			let mut a_val = cpu.read_8(a_ref);

			if !cpu.get_flag(Flag::N) {
				if cpu.get_flag(Flag::C) || a_val > 0x99 {
					a_val = a_val.wrapping_add(0x60);
					cpu.set_flag(Flag::C);
				}
				if cpu.get_flag(Flag::H) || (a_val & 0x0f) > 0x09 {
					a_val = a_val.wrapping_add(0x6);
				}
			} else {
				if cpu.get_flag(Flag::C) {
					a_val = a_val.wrapping_sub(0x60);
				}
				if cpu.get_flag(Flag::H) {
					a_val = a_val.wrapping_sub(0x6);
				}
			}

			cpu.clear_flag(Flag::H);
			cpu.set_flag_to(Flag::Z, a_val == 0);
			cpu.write_8(a_ref, a_val);
		}
		CPL => {
			// Complement A Register
			let current = cpu.read_8(&CPURegister8::A.into());
			cpu.set_flag(Flag::H);
			cpu.set_flag(Flag::N);
			cpu.write_8(&CPURegister8::A.into(), !current);
		}
		SCF => {
			// Set Carry Flag
			cpu.clear_flag(Flag::H);
			cpu.clear_flag(Flag::N);
			cpu.set_flag(Flag::C);
		}
		CCF => {
			// Complement Carry FLag
			cpu.clear_flag(Flag::H);
			cpu.clear_flag(Flag::N);
			cpu.set_flag_to(Flag::C, !cpu.get_flag(Flag::C));
		}
		BIT(bit, value) => {
			let value = cpu.read_8(&value);
			cpu.set_flag_to(Flag::Z, (value >> bit) & 1 == 0);
			cpu.set_flag(Flag::H);
			cpu.clear_flag(Flag::N);
		}
		RES(bit, value) => {
			let current = cpu.read_8(&value);
			cpu.write_8(&value, current & (0xFF ^ (1 << bit)));
		}
		SET(bit, value) => {
			let current = cpu.read_8(&value);
			cpu.write_8(&value, current | (1 << bit));
		}
		ROT(operator, val_ref) => {
			use super::RotShiftOperation::*;
			let value = cpu.read_8(&val_ref);

			let carry_bit = u8::from(cpu.get_flag(Flag::C));

			match operator {
				RLC => {
					cpu.clear_flag(Flag::N);
					cpu.clear_flag(Flag::H);
					cpu.set_flag_to(Flag::C, value & 0b10000000 == 0b10000000);
					cpu.set_flag_to(Flag::Z, value == 0);
					cpu.write_8(&val_ref, value.rotate_left(1));
				}
				RRC => {
					cpu.clear_flag(Flag::N);
					cpu.clear_flag(Flag::H);
					cpu.set_flag_to(Flag::C, value & 1 == 1);
					cpu.set_flag_to(Flag::Z, value == 0);
					cpu.write_8(&val_ref, value.rotate_right(1));
				}
				RL => {
					let result = (value << 1) | carry_bit;
					cpu.clear_flag(Flag::N);
					cpu.clear_flag(Flag::H);
					cpu.set_flag_to(Flag::C, value & 0b10000000 == 0b10000000);
					cpu.set_flag_to(Flag::Z, result == 0);
					cpu.write_8(&val_ref, result);
				}
				RR => {
					let result = ((value >> 1) & 0b01111111) | (carry_bit << 7);
					cpu.set_flag_to(Flag::C, value & 1 != 0);
					cpu.clear_flag(Flag::N);
					cpu.clear_flag(Flag::H);
					cpu.set_flag_to(Flag::Z, result == 0);
					cpu.write_8(&val_ref, result);
				}
				SLA => {
					let result = value << 1;
					cpu.clear_flag(Flag::N);
					cpu.clear_flag(Flag::H);
					cpu.set_flag_to(Flag::C, value & 0b10000000 == 0b10000000);
					cpu.set_flag_to(Flag::Z, result == 0);
					cpu.write_8(&val_ref, result);
				}
				SRA => {
					let result = (value >> 1) | (value & 0b10000000);
					cpu.clear_flag(Flag::N);
					cpu.clear_flag(Flag::H);
					cpu.set_flag_to(Flag::C, value & 1 == 1);
					cpu.set_flag_to(Flag::Z, result == 0);
					cpu.write_8(&val_ref, result);
				}
				SWAP => {
					cpu.clear_flag(Flag::N);
					cpu.clear_flag(Flag::H);
					cpu.clear_flag(Flag::C);
					cpu.set_flag_to(Flag::Z, value == 0);
					cpu.write_8(&val_ref, value.rotate_right(4));
				}
				SRL => {
					let result = value >> 1;
					cpu.set_flag_to(Flag::C, value & 1 != 0);
					cpu.clear_flag(Flag::N);
					cpu.clear_flag(Flag::H);
					cpu.set_flag_to(Flag::Z, result == 0);

					cpu.write_8(&val_ref, result);
				}
			}
		}

		LD_HL_SP_DD(value) => {
			cpu.tick_m_cycles(1);
			cpu.clear_flag(Flag::Z);
			cpu.clear_flag(Flag::N);

			let a_val = cpu.read_16(&CPURegister16::SP.into());
			let b_val = cpu.read_i8(&value) as i16;

			let ub_val = if b_val < 0 {
				((b_val as u16) ^ 0xFFFF).wrapping_sub(1)
			} else {
				b_val as u16
			};

			cpu.set_flag_to(Flag::C, (a_val << 8).wrapping_add(ub_val << 8) < a_val << 8);

			cpu.set_flag_to(
				Flag::H,
				((a_val & 0xf).wrapping_add(ub_val & 0xf) & 0x10) == 0x10,
			);

			cpu.write_16(&CPURegister16::HL.into(), a_val.wrapping_add_signed(b_val));
		}

		LD_A_INC_HL => {
			execute_instruction(
				Instruction::LD_8(CPURegister8::A.into(), CPURegister16::HL.into()),
				cpu,
			);
			let ptr = &CPURegister16::HL.into();
			let ptr_val = cpu.read_16(ptr);
			cpu.write_16(ptr, ptr_val.wrapping_add(1));
		}

		LD_A_DEC_HL => {
			execute_instruction(
				Instruction::LD_8(CPURegister8::A.into(), CPURegister16::HL.into()),
				cpu,
			);
			let ptr = &CPURegister16::HL.into();
			let ptr_val = cpu.read_16(ptr);

			match ptr {
				ValueRefU16::Reg(_) => {}
				_ => {
					cpu.set_flag(Flag::N);
					cpu.set_flag_to(Flag::Z, ptr_val.wrapping_sub(1) == 0);
					cpu.set_flag_to(Flag::H, (((ptr_val & 0xf) - 1) & 0x10) == 0x10);
				}
			}

			cpu.write_16(ptr, ptr_val.wrapping_sub(1));
		}

		LD_INC_HL_A => {
			execute_instruction(
				Instruction::LD_8(CPURegister16::HL.into(), CPURegister8::A.into()),
				cpu,
			);
			let ptr = &CPURegister16::HL.into();
			let ptr_val = cpu.read_16(ptr);
			cpu.write_16(ptr, ptr_val.wrapping_add(1));
		}

		LD_DEC_HL_A => {
			execute_instruction(
				Instruction::LD_8(CPURegister16::HL.into(), CPURegister8::A.into()),
				cpu,
			);

			let ptr = &CPURegister16::HL.into();
			let ptr_val = cpu.read_16(ptr);

			match ptr {
				ValueRefU16::Reg(_) => {}
				_ => {
					cpu.set_flag(Flag::N);
					cpu.set_flag_to(Flag::Z, ptr_val.wrapping_sub(1) == 0);
					cpu.set_flag_to(Flag::H, (((ptr_val & 0xf) - 1) & 0x10) == 0x10);
				}
			}

			cpu.write_16(ptr, ptr_val.wrapping_sub(1));
		}

		RETI => {
			execute_instruction(RET(Condition::ALWAYS), cpu);
			execute_instruction(EI, cpu);
		}
	}
}
