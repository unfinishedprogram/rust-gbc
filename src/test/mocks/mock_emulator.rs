#![allow(dead_code)]
#![allow(unused_variables)]

use crate::emulator::{
	cpu::{
		instruction::{condition::Condition, Instruction},
		values::{ValueRefI8, ValueRefU16, ValueRefU8},
		CPU,
	},
	flags::InterruptFlag,
	memory_mapper::MemoryMapper,
};

pub struct MockEmulator {
	memory: [u8; 0x10000],
	pc: u16,
}
impl Default for MockEmulator {
	fn default() -> Self {
		Self {
			memory: [0; 0x10000],
			pc: 0x100,
		}
	}
}

impl MemoryMapper for MockEmulator {
	fn read(&self, addr: u16) -> u8 {
		self.memory[addr as usize]
	}

	fn write(&mut self, addr: u16, value: u8) {
		self.memory[addr as usize] = value;
	}
}

impl CPU for MockEmulator {
	fn disable_interrupts(&mut self) {}
	fn enable_interrupts(&mut self) {}

	fn next_displacement(&mut self) -> i8 {
		self.next_byte() as i8
	}

	fn next_byte(&mut self) -> u8 {
		self.pc = self.pc.wrapping_add(1);
		self.read(self.pc - 1)
	}

	fn read_8(&mut self, value_ref: &ValueRefU8) -> u8 {
		0
	}
	fn read_i8(&mut self, value_ref: ValueRefI8) -> i8 {
		0
	}
	fn write_8(&mut self, value_ref: &ValueRefU8, value: u8) {}
	fn read_16(&mut self, value_ref: ValueRefU16) -> u16 {
		0
	}
	fn write_16(&mut self, value_ref: ValueRefU16, value: u16) {}

	fn fetch_next_instruction(&mut self) -> Instruction {
		todo!()
	}

	fn check_condition(&self, condition: Condition) -> bool {
		false
	}
	fn check_interrupt(&self, interrupt: InterruptFlag) -> bool {
		false
	}
	fn clear_request(&mut self, interrupt: InterruptFlag) {}
	fn get_interrupt(&mut self) -> Option<Instruction> {
		None
	}
	fn get_next_instruction_or_interrupt(&mut self) -> Instruction {
		todo!()
	}
	fn step(&mut self) {
		todo!()
	}
}
