pub mod flags;
mod gb_stack;
pub mod instruction;
pub mod registers;
mod state;
pub mod values;

use crate::emulator::cpu::flags::Flags;

use super::memory_mapper::MemoryMapper;
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
	fn read_i8(&mut self, value_ref: ValueRefI8) -> i8;
	fn write_8(&mut self, value_ref: &ValueRefU8, value: u8);
	fn read_16(&mut self, value_ref: ValueRefU16) -> u16;
	fn write_16(&mut self, value_ref: ValueRefU16, value: u16);

	fn fetch_next_instruction(&mut self) -> Instruction;

	fn check_condition(&self, condition: Condition) -> bool;
	fn check_interrupt(&self, interrupt: InterruptFlag) -> bool;
	fn clear_request(&mut self, interrupt: InterruptFlag);
	fn get_interrupt(&mut self) -> Option<Instruction>;
	fn get_next_instruction_or_interrupt(&mut self) -> Instruction;
	fn step(&mut self) -> Option<Instruction>;
}

impl CPU for EmulatorState {
	fn disable_interrupts(&mut self) {
		self.cpu_state.interrupt_enable = false;
	}

	fn enable_interrupts(&mut self) {
		// TODO Add delay
		self.cpu_state.interrupt_enable = true;
	}

	fn next_byte(&mut self) -> u8 {
		let value = self.read_8(&ValueRefU8::Mem(ValueRefU16::Reg(
			registers::CPURegister16::PC,
		)));
		self.cpu_state.registers.pc = self.cpu_state.registers.pc.wrapping_add(1);
		return value;
	}

	fn read_8(&mut self, value_ref: &ValueRefU8) -> u8 {
		match value_ref.clone() {
			ValueRefU8::Mem(addr) => {
				self.cycle += 1;
				let index = self.read_16(addr);
				self.read(index)
			}
			ValueRefU8::Reg(reg) => self.cpu_state.registers[reg],
			ValueRefU8::Raw(x) => x,
			ValueRefU8::MemOffset(offset) => {
				let offset_value: u16 = self.read_8(&offset) as u16;
				self.read_8(&ValueRefU8::Mem(ValueRefU16::Raw(offset_value + 0xFF00)))
			}
		}
	}

	fn read_i8(&mut self, value_ref: ValueRefI8) -> i8 {
		match value_ref {
			ValueRefI8::Mem(addr) => self.read(addr) as i8,
			ValueRefI8::Reg(reg) => self.cpu_state.registers[reg] as i8,
			ValueRefI8::Raw(x) => x,
		}
	}

	fn write_8(&mut self, value_ref: &ValueRefU8, value: u8) {
		match value_ref {
			ValueRefU8::Mem(addr) => {
				let index = self.read_16(*addr);
				self.cycle += 1;
				self.write(index, value);
			}
			ValueRefU8::Reg(reg) => self.cpu_state.registers[*reg] = value,
			ValueRefU8::Raw(_) => unreachable!(),
			ValueRefU8::MemOffset(offset) => {
				let offset_value: u16 = self.read_8(&offset) as u16;
				self.write_8(
					&ValueRefU8::Mem(ValueRefU16::Raw(offset_value + 0xFF00)),
					value,
				)
			}
		}
	}

	fn read_16(&mut self, value_ref: ValueRefU16) -> u16 {
		match value_ref {
			ValueRefU16::Mem(i) => u16::from_le_bytes([self.read(i), self.read(i.wrapping_add(1))]),
			ValueRefU16::Reg(reg) => self.cpu_state.registers.get_u16(reg),
			ValueRefU16::Raw(x) => x,
		}
	}

	fn write_16(&mut self, value_ref: ValueRefU16, value: u16) {
		match value_ref {
			ValueRefU16::Mem(i) => {
				let bytes = u16::to_le_bytes(value);
				self.write(i, bytes[0]);
				self.write(i + 1, bytes[1]);
			}
			ValueRefU16::Reg(reg) => self.cpu_state.registers.set_u16(reg, value),
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
			Condition::NZ => !self.get_flag(Flag::Z),
			Condition::Z => self.get_flag(Flag::Z),
			Condition::NC => !self.get_flag(Flag::C),
			Condition::C => self.get_flag(Flag::C),
		}
	}

	fn check_interrupt(&self, interrupt: InterruptFlag) -> bool {
		get_bit_flag(self, BitFlag::InterruptEnable(interrupt))
			&& get_bit_flag(self, BitFlag::InterruptRequest(interrupt))
	}

	fn clear_request(&mut self, interrupt: InterruptFlag) {
		clear_bit_flag(self, BitFlag::InterruptRequest(interrupt));
	}

	fn get_interrupt(&mut self) -> Option<Instruction> {
		use InterruptFlag::*;
		let to_check = [VBlank, LcdStat, Timer, Serial, JoyPad];
		if self.cpu_state.interrupt_enable {
			for interrupt in to_check {
				if self.check_interrupt(interrupt) {
					self.clear_request(interrupt);
					return Some(Instruction::INT(interrupt.clone()));
				}
			}
		}
		return None;
	}

	fn get_next_instruction_or_interrupt(&mut self) -> Instruction {
		return self.fetch_next_instruction();
		// return if let Some(inst) = self.get_interrupt() {
		// 	self.disable_interrupts();
		// 	inst
		// } else {
		// 	self.fetch_next_instruction()
		// };
	}

	fn step(&mut self) -> Option<Instruction> {
		if self.cpu_state.t_states == 0 {
			let instruction = self.get_next_instruction_or_interrupt();
			execute_instruction(instruction.clone(), self);
			return Some(instruction);
		}
		self.cpu_state.t_states -= 1;
		return None;
	}
}
