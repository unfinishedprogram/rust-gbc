pub mod flags;
mod gb_stack;
pub mod instruction;
pub mod registers;
mod state;
pub mod values;

use crate::emulator::cpu::flags::Flags;

use super::memory_mapper::{Source, SourcedMemoryMapper};
use log::warn;
pub use state::CPUState;

use super::flags::*;
use super::state::EmulatorState;
use instruction::{execute::execute_instruction, fetch::fetch_instruction, Instruction};

use registers::CPURegisters;
use values::{ValueRefU16, ValueRefU8};

use self::flags::Flag;
use self::{instruction::condition::Condition, values::ValueRefI8};

pub trait CPU {
	fn disable_interrupts(&mut self);
	fn enable_interrupts(&mut self);
	fn next_byte(&mut self) -> u8;

	fn next_displacement(&mut self) -> i8 {
		self.next_byte() as i8
	}

	fn next_chomp(&mut self) -> u16 {
		let big = self.next_byte();
		let small = self.next_byte();
		u16::from_le_bytes([big, small])
	}

	fn read_8(&mut self, value_ref: &ValueRefU8) -> u8;
	fn read_i8(&mut self, value_ref: &ValueRefI8) -> i8;
	fn write_8(&mut self, value_ref: &ValueRefU8, value: u8);
	fn read_16(&mut self, value_ref: &ValueRefU16) -> u16;
	fn write_16(&mut self, value_ref: &ValueRefU16, value: u16);

	fn fetch_next_instruction(&mut self) -> Instruction;
	fn interrupt_pending(&self) -> bool;

	fn check_condition(&self, condition: Condition) -> bool;
	fn check_interrupt(&self, interrupt: u8) -> bool;
	fn clear_request(&mut self, interrupt: u8);
	fn get_interrupt(&mut self) -> Option<Instruction>;
	fn get_next_instruction_or_interrupt(&mut self) -> Instruction;
	fn step_cpu(&mut self);
}

impl CPU for EmulatorState {
	fn disable_interrupts(&mut self) {
		self.cpu_state.interrupt_enable = false;
	}

	fn enable_interrupts(&mut self) {
		self.cpu_state.interrupt_enable = true;
	}

	fn next_byte(&mut self) -> u8 {
		self.tick_m_cycles(1);
		let value = self.read_from(self.cpu_state.registers.pc, Source::Cpu);
		self.cpu_state.registers.pc = self.cpu_state.registers.pc.wrapping_add(1);
		value
	}

	fn read_8(&mut self, value_ref: &ValueRefU8) -> u8 {
		match value_ref {
			ValueRefU8::Mem(addr) => {
				let index = self.read_16(addr);
				self.tick_m_cycles(1);
				self.read_from(index, Source::Cpu)
			}
			ValueRefU8::Reg(reg) => self.cpu_state.registers[reg.clone()],
			ValueRefU8::Raw(x) => *x,
			ValueRefU8::MemOffset(offset) => {
				let offset_value: u16 = self.read_8(offset) as u16;
				self.read_8(&ValueRefU8::Mem(ValueRefU16::Raw(offset_value | 0xFF00)))
			}
		}
	}

	fn read_i8(&mut self, value_ref: &ValueRefI8) -> i8 {
		match value_ref {
			ValueRefI8::Mem(addr) => self.read_from(*addr, Source::Cpu) as i8,
			ValueRefI8::Reg(reg) => self.cpu_state.registers[reg.clone()] as i8,
			ValueRefI8::Raw(x) => *x,
		}
	}

	fn write_8(&mut self, value_ref: &ValueRefU8, value: u8) {
		match value_ref {
			ValueRefU8::Mem(addr) => {
				let index = self.read_16(addr);
				self.tick_m_cycles(1);
				self.write_from(index, value, Source::Cpu);
			}
			ValueRefU8::Reg(reg) => self.cpu_state.registers[reg.clone()] = value,
			ValueRefU8::MemOffset(offset) => {
				let offset_value: u16 = self.read_8(offset) as u16;
				self.write_8(
					&ValueRefU8::Mem(ValueRefU16::Raw(offset_value | 0xFF00)),
					value,
				)
			}
			ValueRefU8::Raw(_) => unreachable!(),
		}
	}

	fn read_16(&mut self, value_ref: &ValueRefU16) -> u16 {
		match value_ref {
			ValueRefU16::Mem(i) => u16::from_le_bytes([
				self.read_from(*i, Source::Cpu),
				self.read_from(i + 1, Source::Cpu),
			]),
			ValueRefU16::Reg(reg) => self.cpu_state.registers.get_u16(reg.clone()),
			ValueRefU16::Raw(x) => *x,
		}
	}

	fn write_16(&mut self, value_ref: &ValueRefU16, value: u16) {
		match value_ref {
			ValueRefU16::Mem(i) => {
				let bytes = u16::to_le_bytes(value);
				self.tick_m_cycles(2);
				self.write_from(*i, bytes[0], Source::Cpu);
				self.write_from(*i + 1, bytes[1], Source::Cpu);
			}
			ValueRefU16::Reg(reg) => self.cpu_state.registers.set_u16(reg.clone(), value),
			ValueRefU16::Raw(_) => unreachable!(),
		}
	}

	fn fetch_next_instruction(&mut self) -> Instruction {
		fetch_instruction(self)
	}

	fn check_condition(&self, condition: Condition) -> bool {
		use Condition::*;

		match condition {
			ALWAYS => true,
			NZ => !self.get_flag(Flag::Z),
			Z => self.get_flag(Flag::Z),
			NC => !self.get_flag(Flag::C),
			C => self.get_flag(Flag::C),
		}
	}

	fn check_interrupt(&self, interrupt: u8) -> bool {
		let enabled = self.read_from(INTERRUPT_ENABLE, Source::Cpu);
		let requested = self.read_from(INTERRUPT_REQUEST, Source::Cpu);
		enabled & requested & interrupt == interrupt
	}

	fn clear_request(&mut self, interrupt: u8) {
		let request_value = self.read_from(INTERRUPT_REQUEST, Source::Cpu);
		self.write_from(INTERRUPT_REQUEST, request_value & !interrupt, Source::Cpu);
	}

	fn interrupt_pending(&self) -> bool {
		self.read_from(INTERRUPT_ENABLE, Source::Cpu)
			& self.read_from(INTERRUPT_REQUEST, Source::Cpu)
			!= 0
	}

	fn get_interrupt(&mut self) -> Option<Instruction> {
		if !self.cpu_state.interrupt_enable {
			return None;
		}

		for interrupt in [
			INT_V_BLANK,
			INT_LCD_STAT,
			INT_TIMER,
			INT_SERIAL,
			INT_JOY_PAD,
		] {
			if self.check_interrupt(interrupt) {
				self.clear_request(interrupt);
				return Some(Instruction::INT(interrupt));
			}
		}
		None
	}

	fn get_next_instruction_or_interrupt(&mut self) -> Instruction {
		if let Some(interrupt_instruction) = self.get_interrupt() {
			return interrupt_instruction;
		}
		self.fetch_next_instruction()
	}

	fn step_cpu(&mut self) {
		if self.halted {
			if self.interrupt_pending() {
				self.halted = false;
			} else {
				self.tick_m_cycles(1);
			}
		} else {
			let instruction = self.get_next_instruction_or_interrupt();
			warn!("{instruction:?}");
			execute_instruction(instruction, self);
		}
	}
}
