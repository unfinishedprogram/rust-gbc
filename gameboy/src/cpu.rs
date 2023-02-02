use crate::io_registers::{IE, IF};

use self::flags::Flags;

mod condition;
pub mod flags;
mod cpu_stack;
pub mod instruction;
pub mod registers;
mod state;
mod values;

use super::memory_mapper::{Source, SourcedMemoryMapper};
pub use state::CPUState;

use super::state::Gameboy;
use instruction::{execute::execute_instruction, fetch::fetch_instruction, Instruction};

use registers::CPURegisters;
use values::{ValueRefU16, ValueRefU8};

use self::condition::Condition;

pub trait CPU<M: SourcedMemoryMapper> {
	fn disable_interrupts(&mut self) {
		self.cpu_state_mut().ime = false;
	}

	fn enable_interrupts(&mut self) {
		self.cpu_state_mut().ie_next = true;
	}

	fn next_displacement(&mut self) -> i8 {
		self.next_byte() as i8
	}

	fn next_byte(&mut self) -> u8 {
		let value = self.read_8(&ValueRefU8::Mem(self.cpu_state().registers.pc.into()));
		self.cpu_state_mut().registers.pc = self.cpu_state().registers.pc.wrapping_add(1);
		value
	}

	fn next_chomp(&mut self) -> u16 {
		let pc = self.cpu_state().registers.pc;
		let high = self.read_8(&ValueRefU8::Mem(pc.into()));
		let low = self.read_8(&ValueRefU8::Mem((pc.wrapping_add(1)).into()));

		self.cpu_state_mut().registers.pc = self.cpu_state().registers.pc.wrapping_add(2);

		u16::from_le_bytes([high, low])
	}

	fn read_8(&mut self, value_ref: &ValueRefU8) -> u8 {
		match value_ref {
			ValueRefU8::Mem(addr) => {
				self.tick_m_cycles(1);
				let index = self.read_16(addr);
				let value = self.get_memory_mapper_mut().read_from(index, Source::Cpu);
				value
			}
			ValueRefU8::Reg(reg) => self.cpu_state().registers[*reg],
			ValueRefU8::Raw(x) => *x,
			ValueRefU8::MemOffsetRaw(offset) => self.read_8(&ValueRefU8::Mem(ValueRefU16::Raw(
				(*offset as u16) | 0xFF00,
			))),
			ValueRefU8::MemOffsetReg(reg) => {
				let offset = self.cpu_state().registers[*reg];
				self.read_8(&ValueRefU8::Mem(ValueRefU16::Raw((offset as u16) | 0xFF00)))
			}
		}
	}

	fn write_8(&mut self, value_ref: &ValueRefU8, value: u8) {
		match value_ref {
			ValueRefU8::Mem(addr) => {
				self.tick_m_cycles(1);
				let index = self.read_16(addr);
				self.get_memory_mapper_mut()
					.write_from(index, value, Source::Cpu);
			}
			ValueRefU8::Reg(reg) => self.cpu_state_mut().registers[*reg] = value,
			ValueRefU8::MemOffsetRaw(offset) => self.write_8(
				&ValueRefU8::Mem(ValueRefU16::Raw((*offset as u16) | 0xFF00)),
				value,
			),
			ValueRefU8::MemOffsetReg(reg) => {
				let offset = self.cpu_state_mut().registers[*reg];
				self.write_8(
					&ValueRefU8::Mem(ValueRefU16::Raw((offset as u16) | 0xFF00)),
					value,
				);
			}
			ValueRefU8::Raw(_) => unreachable!(),
		}
	}

	fn read_16(&mut self, value_ref: &ValueRefU16) -> u16 {
		match value_ref {
			ValueRefU16::Mem(i) => {
				self.tick_m_cycles(1);
				let upper = self.get_memory_mapper_mut().read_from(i + 1, Source::Cpu);
				self.tick_m_cycles(1);
				let lower = self.get_memory_mapper_mut().read_from(*i, Source::Cpu);
				u16::from_le_bytes([lower, upper])
			}
			ValueRefU16::Reg(reg) => self.cpu_state().registers.get_u16(*reg),
			ValueRefU16::Raw(x) => *x,
		}
	}

	fn write_16(&mut self, value_ref: &ValueRefU16, value: u16) {
		match value_ref {
			ValueRefU16::Mem(i) => {
				let bytes = u16::to_le_bytes(value);
				self.tick_m_cycles(1);
				self.get_memory_mapper_mut()
					.write_from(*i + 1, bytes[1], Source::Cpu);
				self.tick_m_cycles(1);
				self.get_memory_mapper_mut()
					.write_from(*i, bytes[0], Source::Cpu);
			}
			ValueRefU16::Reg(reg) => self.cpu_state_mut().registers.set_u16(*reg, value),
			ValueRefU16::Raw(_) => unreachable!(),
		}
	}

	fn check_condition(&self, condition: Condition) -> bool {
		use Condition::*;

		match condition {
			ALWAYS => true,
			NZ => !self.cpu_state().get_flag(flags::Z),
			Z => self.cpu_state().get_flag(flags::Z),
			NC => !self.cpu_state().get_flag(flags::C),
			C => self.cpu_state().get_flag(flags::C),
		}
	}

	fn fetch_next_instruction(&mut self) -> Instruction where Self:Sized {
		fetch_instruction(self)
	}

	fn get_next_instruction_or_interrupt(&mut self) -> Instruction where Self:Sized {
		if let Some(int) = self.fetch_next_interrupt() {
			Instruction::INT(int)
		} else {
			self.fetch_next_instruction()
		}
	}

	fn interrupt_pending(&self) -> bool {
		// No interrupts can be pending if there are none enabled
		if self.cpu_state().ie_register == 0 {
			return false;
		};

		self.cpu_state().ie_register & self.get_memory_mapper().read(IF) != 0
	}

	fn check_interrupt(&self, interrupt: u8) -> bool {
		self.get_memory_mapper().read(IE) & self.get_memory_mapper().read(IF) & interrupt != 0
	}

	fn clear_request(&mut self, interrupt: u8) {
		let flag = self.get_memory_mapper().read(IF);
		self.get_memory_mapper_mut().write(IF, flag & !interrupt);
	}

	
	fn fetch_next_interrupt(&mut self) -> Option<u8> {
		if !self.cpu_state().ime {
			return None;
		}

		let requests = self.cpu_state_mut().ie_register & self.get_memory_mapper().read(IF);

		if requests == 0 {
			return None;
		};

		for index in 0..5 {
			let interrupt = 1 << index;
			if requests & interrupt != 0 {
				self.clear_request(interrupt);
				return Some(interrupt);
			}
		}
		None
	}

	fn step_cpu(&mut self) where Self:Sized {
		if self.cpu_state().halted {
			if self.interrupt_pending() {
				self.cpu_state_mut().halted = false;
			} else {
				self.tick_m_cycles(1);
			}
		} else {
			let instruction = self.get_next_instruction_or_interrupt();
			execute_instruction(self, instruction);
		}

		if self.cpu_state().ie_next {
			self.cpu_state_mut().ime = true;
			self.cpu_state_mut().ie_next = false;
		}
	}

	fn exec_stop(&mut self) {}
	fn tick_m_cycles(&mut self, _m_cycles: u32) {}
	fn cpu_state(&self) -> &CPUState;
	fn cpu_state_mut(&mut self) -> &mut CPUState;
	fn get_memory_mapper_mut(&mut self) -> &mut M;
	fn get_memory_mapper(&self) -> &M;
}


impl CPU<Gameboy> for Gameboy {
	fn get_memory_mapper_mut(&mut self) -> &mut Gameboy {
		self
	}

	fn get_memory_mapper(&self) -> & Gameboy {
		self
	}

	fn cpu_state(&self) -> &CPUState {
		&self.cpu_state
	}

	fn cpu_state_mut(&mut self) -> &mut CPUState {
		&mut self.cpu_state
	}

	fn tick_m_cycles(&mut self, m_cycles: u32) {
		Gameboy::tick_m_cycles(self, m_cycles)
	}

	fn exec_stop(&mut self) {
		match &mut self.mode {
			crate::state::GameboyMode::GBC(state) => {
				self.speed_switch_delay = 2050;
				state.perform_speed_switch();
			}
			_ => {}
		}
	}
}
