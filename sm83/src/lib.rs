mod bits;
mod cpu;
pub mod instruction;
pub mod memory_mapper;
pub mod registers;
mod stack;
mod state;
mod values;
use instruction::Instruction;
use memory_mapper::{Source, SourcedMemoryMapper};
pub use state::CPUState;

use values::{ValueRefU16, ValueRefU8};

use cpu::condition::Condition;
pub use cpu::flags::{self, Flags};

pub use cpu::condition;

#[cfg(test)]
mod test;

pub trait SM83<M: SourcedMemoryMapper> {
	fn debug(&self) -> bool {
		false
	}
	fn disable_interrupts(&mut self) {
		self.cpu_state_mut().interrupt_master_enable = false;
		self.cpu_state_mut().ie_next = false;
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
				let lsb = self.get_memory_mapper_mut().read_from(*i, Source::Cpu);
				self.tick_m_cycles(1);
				let msb = self.get_memory_mapper_mut().read_from(i + 1, Source::Cpu);
				u16::from_le_bytes([lsb, msb])
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
			Always => true,
			NZ => !self.cpu_state().get_flag(flags::cpu::Z),
			Z => self.cpu_state().get_flag(flags::cpu::Z),
			NC => !self.cpu_state().get_flag(flags::cpu::C),
			C => self.cpu_state().get_flag(flags::cpu::C),
		}
	}

	fn fetch_next_instruction(&mut self) -> Instruction
	where
		Self: Sized,
	{
		instruction::fetch(self)
	}

	fn get_next_instruction_or_interrupt(&mut self) -> Instruction
	where
		Self: Sized,
	{
		if let Some(int) = self.cpu_state_mut().fetch_next_interrupt() {
			Instruction::INT(int)
		} else {
			self.fetch_next_instruction()
		}
	}

	fn step_cpu(&mut self)
	where
		Self: Sized,
	{
		if self.cpu_state().halted {
			if self.cpu_state().interrupt_pending() {
				self.cpu_state_mut().halted = false;
			} else {
				self.tick_m_cycles(1);
			}
		} else {
			let pc = self.cpu_state().registers.pc;
			let instruction = self.get_next_instruction_or_interrupt();
			if self.debug() {
				log::info!("[{pc:04X}]{instruction:?}");
			}
			instruction::execute(self, instruction);
		}
	}

	fn exec_stop(&mut self) {}
	fn tick_m_cycles(&mut self, m_cycles: u32) {
		self.cpu_state_mut().tick_ie_delay();
		self.on_m_cycle(m_cycles);
	}
	fn on_m_cycle(&mut self, m_cycles: u32) {
		_ = m_cycles
	}
	fn cpu_state(&self) -> &CPUState;
	fn cpu_state_mut(&mut self) -> &mut CPUState;
	fn get_memory_mapper_mut(&mut self) -> &mut M;
	fn get_memory_mapper(&self) -> &M;
}
