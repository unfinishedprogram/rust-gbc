pub mod flags;
mod gb_stack;
mod instruction;
pub mod registers;
pub mod values;

use instruction::{execute::execute_instruction, get_instruction, opcode::Opcode, Instruction};
use registers::CPURegisters;
use values::{as_u16, ValueRefU16, ValueRefU8};

use crate::cpu::flags::{Flag, Flags};

use self::{instruction::Condition, values::ValueRefI8};

#[derive(Debug, serde::Serialize)]
pub struct Cpu {
	pub registers: CPURegisters,
	#[serde(serialize_with = "<[_]>::serialize")]
	pub memory: [u8; 0xFFFF],
}

impl Cpu {
	pub fn new() -> Cpu {
		Cpu {
			registers: CPURegisters::new(),
			memory: [0xFF; 0xFFFF],
		}
	}

	pub fn next_byte(&mut self) -> u8 {
		self.registers.pc += 1;
		self.memory[(self.registers.pc - 1) as usize]
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
			ValueRefU8::Mem(addr) => self.memory[self.read_16(addr) as usize],
			ValueRefU8::Reg(reg) => self.registers.get_u8(reg),
			ValueRefU8::Raw(x) => x,
		}
	}

	pub fn read_i8(&self, value_ref: ValueRefI8) -> i8 {
		match value_ref {
			ValueRefI8::Mem(i) => self.memory[i as usize] as i8,
			ValueRefI8::Reg(reg) => self.registers.get_u8(reg) as i8,
			ValueRefI8::Raw(x) => x,
		}
	}

	pub fn write_8(&mut self, value_ref: ValueRefU8, value: u8) {
		match value_ref {
			ValueRefU8::Mem(addr) => self.memory[self.read_16(addr) as usize] = value,
			ValueRefU8::Reg(reg) => self.registers.set_u8(reg, value),
			ValueRefU8::Raw(_) => unreachable!(),
		}
	}

	pub fn read_16(&self, value_ref: ValueRefU16) -> u16 {
		match value_ref {
			ValueRefU16::Mem(i) => as_u16([self.memory[i as usize], self.memory[i as usize + 1]]),
			ValueRefU16::Reg(reg) => self.registers.get_u16(reg),
			ValueRefU16::Raw(x) => x,
		}
	}

	pub fn write_16(&mut self, value_ref: ValueRefU16, value: u16) {
		match value_ref {
			ValueRefU16::Mem(i) => {
				self.memory[i as usize] = (value >> 8) as u8;
				self.memory[i as usize + 1] = (value & 0xFF) as u8;
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
		for i in 0..rom.len() {
			self.memory[i] = rom[i];
		}
	}

	pub fn load_boot_rom(&mut self, rom: &[u8]) {
		for i in 0..rom.len() {
			self.memory[i] = rom[i];
		}
	}
}
