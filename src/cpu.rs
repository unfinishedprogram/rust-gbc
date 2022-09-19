pub mod flags;
mod gb_stack;
mod instruction;
pub mod registers;
pub mod values;

use std::cell::RefCell;
use std::rc::Rc;

use instruction::{execute::execute_instruction, get_instruction, opcode::Opcode, Instruction};
use registers::CPURegisters;
use values::{as_u16, ValueRefU16, ValueRefU8};

use crate::{
	cpu::flags::{Flag, Flags},
	memory::Memory,
};

use self::{instruction::Condition, values::ValueRefI8};

pub struct Cpu {
	pub registers: CPURegisters,
	pub memory: Rc<RefCell<Memory>>,
}

impl Cpu {
	pub fn new(memory: Rc<RefCell<Memory>>) -> Cpu {
		Cpu {
			memory,
			registers: CPURegisters::new(),
		}
	}

	pub fn next_byte(&mut self) -> u8 {
		self.registers.pc += 1;
		self.memory.borrow()[self.registers.pc - 1]
	}

	pub fn next_displacement(&mut self) -> i8 {
		self.next_byte() as i8
	}

	pub fn next_chomp(&mut self) -> u16 {
		self.registers.pc += 2;
		self.read_16(ValueRefU16::Mem(self.registers.pc - 2))
	}

	pub fn read_8(&self, value_ref: ValueRefU8) -> u8 {
		match value_ref {
			ValueRefU8::Mem(addr) => self.memory.borrow()[self.read_16(addr)],
			ValueRefU8::Reg(reg) => self.registers.get_u8(reg),
			ValueRefU8::Raw(x) => x,
		}
	}

	pub fn read_i8(&self, value_ref: ValueRefI8) -> i8 {
		match value_ref {
			ValueRefI8::Mem(i) => self.memory.borrow()[i] as i8,
			ValueRefI8::Reg(reg) => self.registers.get_u8(reg) as i8,
			ValueRefI8::Raw(x) => x,
		}
	}

	pub fn write_8(&mut self, value_ref: ValueRefU8, value: u8) {
		match value_ref {
			ValueRefU8::Mem(addr) => self.memory.borrow_mut()[self.read_16(addr)] = value,
			ValueRefU8::Reg(reg) => self.registers.set_u8(reg, value),
			ValueRefU8::Raw(_) => unreachable!(),
		}
	}

	pub fn read_16(&self, value_ref: ValueRefU16) -> u16 {
		match value_ref {
			ValueRefU16::Mem(i) => {
				let mem = self.memory.borrow();
				as_u16([mem[i], mem[i + 1]])
			}
			ValueRefU16::Reg(reg) => self.registers.get_u16(reg),
			ValueRefU16::Raw(x) => x,
		}
	}

	pub fn write_16(&mut self, value_ref: ValueRefU16, value: u16) {
		match value_ref {
			ValueRefU16::Mem(i) => {
				let mut mem = self.memory.borrow_mut();
				mem[i + 1] = (value >> 8) as u8;
				mem[i] = (value & 0xFF) as u8;
			}
			ValueRefU16::Reg(reg) => self.registers.set_u16(reg, value),
			ValueRefU16::Raw(_) => unreachable!(),
		}
	}

	pub fn get_next_instruction(&mut self) -> Instruction {
		let opcode: Opcode = Opcode::from(self.next_byte());
		get_instruction(self, opcode)
	}

	pub fn check_condition(&self, condition: Condition) -> bool {
		use Condition::*;

		match condition {
			ALWAYS => true,
			Condition::NZ => !self.get_flag(Flag::Z),
			Condition::Z => self.get_flag(Flag::Z),
			Condition::NC => !self.get_flag(Flag::C),
			Condition::C => self.get_flag(Flag::C),
		}
	}

	pub fn execute_next_instruction(&mut self) -> Instruction {
		let instruction = self.get_next_instruction().clone();
		execute_instruction(instruction.clone(), self);
		return instruction;
	}

	pub fn load_cartridge(&mut self, rom: &[u8]) {
		let mut mem = self.memory.borrow_mut();

		for i in 0..rom.len() {
			mem[i as u16] = rom[i];
		}
	}

	pub fn load_boot_rom(&mut self, rom: &[u8]) {
		let mut mem = self.memory.borrow_mut();

		for i in 0..rom.len() {
			mem[i as u16] = rom[i];
		}
	}
}
