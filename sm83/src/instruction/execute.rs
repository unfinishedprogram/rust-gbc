use crate::{
	bits::*,
	flags::cpu::{C, H, N, Z},
	instruction::ALUOperation,
	registers::{Addressable, CPURegister16, CPURegister8},
	stack::CPUStack,
	values::{ValueRefI8, ValueRefU16},
	SM83,
};

use super::{
	Condition,
	Instruction::{self, *},
	RotShiftOperation,
};

pub trait Execute {
	fn execute(&mut self, instruction: Instruction) -> Instruction;
}

use std::ops::{BitAnd, BitOr, BitXor};

impl<T: SM83> Execute for T {
	fn execute(&mut self, instruction: Instruction) -> Instruction {
		let cpu = self;
		match instruction {
			NOP => {}
			INT => {
				cpu.tick_m_cycles(2);
				let current_pc = cpu.read_16(CPURegister16::PC.into());
				cpu.push_u8((current_pc >> 8) as u8);
				if let Some(interrupt) = cpu.cpu_state().get_pending_interrupt() {
					cpu.write_16(CPURegister16::PC.into(), interrupt.jump_addr());
					cpu.tick_m_cycles(1);
					cpu.cpu_state_mut().clear_interrupt_request(interrupt as u8);
				} else {
					cpu.write_16(CPURegister16::PC.into(), 0x0);
				}
				cpu.push_u8(current_pc as u8);
				cpu.disable_interrupts();
			}

			LDH(to, from) => {
				let val = cpu.read_8(from);
				cpu.write_8(to, val);
			}

			LD_8(to, from) => {
				let val = cpu.read_8(from);
				cpu.write_8(to, val);
			}

			LD_16(to, from) => {
				if matches!((to, from), (ValueRefU16::Reg(_), ValueRefU16::Reg(_))) {
					cpu.tick_m_cycles(1);
				}

				let val = cpu.read_16(from);
				cpu.write_16(to, val);
			}

			INC_8(ptr) => {
				let val = cpu.read_8(ptr);
				cpu.set_flag_to(Z, val.wrapping_add(1) == 0);
				cpu.clear_flag(N);
				cpu.set_flag_to(H, ((val & 0xF).wrapping_add(1) & 0x10) == 0x10);
				cpu.write_8(ptr, val.wrapping_add(1));
			}

			INC_16(ptr) => {
				cpu.tick_m_cycles(1);
				let ptr_val = cpu.read_16(ptr);
				cpu.write_16(ptr, ptr_val.wrapping_add(1));
			}

			DEC_8(ptr) => {
				let val = cpu.read_8(ptr);
				cpu.set_flag_to(Z, val.wrapping_sub(1) == 0);
				cpu.set_flag(N);
				cpu.set_flag_to(H, ((val & 0xF).wrapping_sub(1 & 0xF) & 0x10) == 0x10);
				cpu.write_8(ptr, val.wrapping_sub(1));
			}

			DEC_16(ptr) => {
				cpu.tick_m_cycles(1);

				let ptr_val = cpu.read_16(ptr);

				match ptr {
					ValueRefU16::Reg(_) => {}
					_ => {
						cpu.set_flag(N);
						cpu.set_flag_to(Z, ptr_val.wrapping_sub(1) == 0);
						cpu.set_flag_to(H, (((ptr_val & 0xF) - 1) & 0x10) == 0x10);
					}
				}

				cpu.write_16(ptr, ptr_val.wrapping_sub(1));
			}

			STOP => cpu.exec_stop(),
			ERROR(_) => {}

			JR(condition, ValueRefI8(offset)) => {
				if cpu.check_condition(condition) {
					let addr = cpu
						.read_16(CPURegister16::PC.into())
						.wrapping_add_signed(offset as i16);

					cpu.write_16(CPURegister16::PC.into(), addr);
					cpu.tick_m_cycles(1);
				}
			}

			JP(condition, location) => {
				if cpu.check_condition(condition) {
					let loc_val = cpu.read_16(location);
					cpu.write_16(CPURegister16::PC.into(), loc_val);
					if !matches!(location, ValueRefU16::Reg(CPURegister16::HL)) {
						cpu.tick_m_cycles(1);
					}
				}
			}

			ADD_16(a_ref, b_ref) => {
				cpu.tick_m_cycles(1);

				let a_val = cpu.read_16(a_ref);
				let b_val = cpu.read_16(b_ref);

				cpu.set_flag_to(
					H,
					(((a_val & 0xFFF).wrapping_add(b_val & 0xFFF)) & 0x1000) == 0x1000,
				);
				cpu.clear_flag(N);
				cpu.set_flag_to(C, a_val.wrapping_add(b_val) < a_val);
				cpu.write_16(a_ref, a_val.wrapping_add(b_val));
			}

			ADD_SIGNED(a_ref, ValueRefI8(b_ref)) => {
				let a_val = cpu.read_16(a_ref);
				let b_val = b_ref as i16 as u16;

				cpu.set_flag_to(H, u16::test_add_carry_bit(3, a_val, b_val));
				cpu.set_flag_to(C, u16::test_add_carry_bit(7, a_val, b_val));
				cpu.clear_flag(Z);
				cpu.clear_flag(N);

				cpu.write_16(a_ref, a_val.wrapping_add(b_val));
				cpu.tick_m_cycles(2);
			}

			ALU_OP_8(op, from) => {
				let a_val = cpu.read_8(CPURegister8::A.into());
				let b_val = cpu.read_8(from);
				let carry = u8::from(cpu.get_flag(C));

				let result = match op {
					ALUOperation::ADD => {
						cpu.clear_flag(N);
						cpu.set_flag_to(H, (a_val & 0xF).wrapping_add(b_val & 0xF) & 0x10 == 0x10);
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
						cpu.clear_flag(N);
						cpu.set_flag(H);
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
						cpu.set_flag(N);
						cpu.set_flag_to(C, b_val > a_val);
						cpu.set_flag_to(Z, a_val == b_val);
						cpu.set_flag_to(H, (a_val & 0xF).wrapping_sub(b_val & 0xF) & 0x10 == 0x10);
						a_val
					}
				};

				match op {
					ALUOperation::CP => {}
					_ => {
						cpu.write_8(CPURegister8::A.into(), result);
						cpu.set_flag_to(Z, result == 0);
					}
				}
			}
			HALT => cpu.cpu_state_mut().halted = true,
			CALL(condition, location) => {
				let loc_value = cpu.read_16(location);
				if cpu.check_condition(condition) {
					cpu.tick_m_cycles(1);
					let current_pc = cpu.cpu_state().read(CPURegister16::PC);
					cpu.push(current_pc);
					cpu.cpu_state_mut().write(CPURegister16::PC, loc_value);
				}
			}
			POP(value_ref) => {
				let val = cpu.pop();
				cpu.write_16(value_ref.into(), val);
			}
			PUSH(value_ref) => {
				let value = cpu.read_16(value_ref.into());
				cpu.tick_m_cycles(1);
				cpu.push(value)
			}
			RET(condition) => {
				if !matches!(condition, Condition::Always) {
					cpu.tick_m_cycles(1);
				}
				if cpu.check_condition(condition) {
					let pc = cpu.pop();
					cpu.cpu_state_mut().write(CPURegister16::PC, pc);
					cpu.tick_m_cycles(1);
				}
			}
			RST(addr) => {
				cpu.tick_m_cycles(1);
				let current_pc = cpu.read_16(CPURegister16::PC.into());
				cpu.push(current_pc);
				let new_pc = cpu.read_16(addr);
				cpu.write_16(CPURegister16::PC.into(), new_pc);
			}
			DI => {
				cpu.disable_interrupts();
			}
			EI => {
				cpu.enable_interrupts();
			}
			RLCA => {
				let value = cpu.read_8(CPURegister8::A.into());
				cpu.clear_flag(N);
				cpu.clear_flag(H);
				cpu.clear_flag(Z);
				cpu.set_flag_to(C, value & BIT_7 == BIT_7);
				cpu.write_8(CPURegister8::A.into(), value.rotate_left(1));
			}
			RRCA => {
				let value = cpu.read_8(CPURegister8::A.into());
				cpu.clear_flag(N);
				cpu.clear_flag(H);
				cpu.clear_flag(Z);
				cpu.set_flag_to(C, value & BIT_0 == BIT_0);
				cpu.write_8(CPURegister8::A.into(), value.rotate_right(1));
			}

			RLA => {
				cpu.execute(Instruction::ROT(
					super::RotShiftOperation::RL,
					CPURegister8::A.into(),
				));
				cpu.clear_flag(Z);
			}
			RRA => {
				cpu.execute(Instruction::ROT(
					super::RotShiftOperation::RR,
					CPURegister8::A.into(),
				));
				cpu.clear_flag(Z);
				cpu.clear_flag(H);
				cpu.clear_flag(N);
			}
			DAA => {
				let mut a_val = cpu.cpu_state().read(CPURegister8::A);
				let mut carry = false;

				if !cpu.get_flag(N) {
					if cpu.get_flag(C) || a_val > 0x99 {
						a_val = a_val.wrapping_add(0x60);
						carry = true;
					}
					if cpu.get_flag(H) || a_val & 0x0f > 0x09 {
						a_val = a_val.wrapping_add(0x06);
					}
				} else if cpu.get_flag(C) {
					carry = true;
					a_val = a_val.wrapping_add(if cpu.get_flag(H) { 0x9a } else { 0xa0 });
				} else if cpu.get_flag(H) {
					a_val = a_val.wrapping_add(0xfa);
				}

				cpu.set_flag_to(Z, a_val == 0);
				cpu.clear_flag(H);
				cpu.set_flag_to(C, carry);
				cpu.cpu_state_mut().write(CPURegister8::A, a_val);
			}
			CPL => {
				// Complement A Register
				let current = cpu.read_8(CPURegister8::A.into());
				cpu.set_flag(H);
				cpu.set_flag(N);
				cpu.write_8(CPURegister8::A.into(), !current);
			}
			SCF => {
				// Set Carry Flag
				cpu.clear_flag(H);
				cpu.clear_flag(N);
				cpu.set_flag(C);
			}
			CCF => {
				// Complement Carry FLag
				cpu.clear_flag(H);
				cpu.clear_flag(N);
				let f = cpu.get_flag(C);
				cpu.set_flag_to(C, !f);
			}
			BIT(bit, value) => {
				let value = cpu.read_8(value);
				cpu.set_flag_to(Z, (value >> bit) & 1 == 0);
				cpu.set_flag(H);
				cpu.clear_flag(N);
			}
			RES(bit, value) => {
				let current = cpu.read_8(value);
				cpu.write_8(value, current & (0xFF ^ (1 << bit)));
			}
			SET(bit, value) => {
				let current = cpu.read_8(value);
				cpu.write_8(value, current | (1 << bit));
			}
			ROT(operator, val_ref) => {
				let value = cpu.read_8(val_ref);
				let carry_bit = u8::from(cpu.get_flag(C));

				let result = match operator {
					RotShiftOperation::RLC => value.rotate_left(1),
					RotShiftOperation::RRC => value.rotate_right(1),
					RotShiftOperation::RL => (value << 1) | carry_bit,
					RotShiftOperation::RR => ((value >> 1) & 0b01111111) | (carry_bit << 7),
					RotShiftOperation::SLA => value << 1,
					RotShiftOperation::SRA => (value >> 1) | (value & BIT_7),
					RotShiftOperation::SWAP => value.rotate_right(4),
					RotShiftOperation::SRL => value >> 1,
				};

				cpu.clear_flag(N);
				cpu.clear_flag(H);
				cpu.set_flag_to(Z, result == 0);
				cpu.set_flag_to(
					C,
					match operator {
						RotShiftOperation::RLC | RotShiftOperation::RL | RotShiftOperation::SLA => {
							value & BIT_7 == BIT_7
						}
						RotShiftOperation::SRL
						| RotShiftOperation::RRC
						| RotShiftOperation::RR
						| RotShiftOperation::SRA => value & BIT_0 == BIT_0,
						RotShiftOperation::SWAP => false,
					},
				);

				cpu.write_8(val_ref, result);
			}

			LD_HL_SP_DD(ValueRefI8(b_ref)) => {
				cpu.tick_m_cycles(1);
				cpu.clear_flag(Z);
				cpu.clear_flag(N);

				let a_val = cpu.read_16(CPURegister16::SP.into());
				let b_val = b_ref as i16;

				let (_, carry) =
					((a_val & 0xFF) as u8).overflowing_add(((b_ref as u16) & 0xFF) as u8);

				cpu.set_flag_to(C, carry);

				cpu.set_flag_to(
					H,
					((a_val & 0xF).wrapping_add((b_ref as u16) & 0xF) & 0x10) == 0x10,
				);

				cpu.write_16(CPURegister16::HL.into(), a_val.wrapping_add_signed(b_val));
			}

			LD_A_INC_HL => {
				cpu.execute(Instruction::LD_8(
					CPURegister8::A.into(),
					CPURegister16::HL.into(),
				));
				let ptr = CPURegister16::HL.into();
				let ptr_val = cpu.read_16(ptr);
				cpu.write_16(ptr, ptr_val.wrapping_add(1));
			}

			LD_A_DEC_HL => {
				cpu.execute(Instruction::LD_8(
					CPURegister8::A.into(),
					CPURegister16::HL.into(),
				));
				let ptr = CPURegister16::HL.into();
				let ptr_val = cpu.read_16(ptr);
				cpu.write_16(ptr, ptr_val.wrapping_sub(1));
			}

			LD_INC_HL_A => {
				cpu.execute(Instruction::LD_8(
					CPURegister16::HL.into(),
					CPURegister8::A.into(),
				));
				let ptr = CPURegister16::HL.into();
				let ptr_val = cpu.read_16(ptr);
				cpu.write_16(ptr, ptr_val.wrapping_add(1));
			}

			LD_DEC_HL_A => {
				cpu.execute(Instruction::LD_8(
					CPURegister16::HL.into(),
					CPURegister8::A.into(),
				));

				let ptr = CPURegister16::HL.into();
				let ptr_val = cpu.read_16(ptr);

				cpu.write_16(ptr, ptr_val.wrapping_sub(1));
			}

			RETI => {
				cpu.execute(EI);
				cpu.execute(RET(Condition::Always));
			}
		}
		instruction
	}
}
