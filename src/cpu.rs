pub mod flags;
mod gb_stack;
pub mod instruction;
pub mod registers;
pub mod values;

use std::cell::RefCell;
use std::rc::Rc;

use instruction::{execute::execute_instruction, get_instruction, opcode::Opcode, Instruction};
use registers::{CPURegister16, CPURegisters};
use values::{as_u16, ValueRefU16, ValueRefU8};

use crate::{
	cartridge::CartridgeData,
	cpu::flags::{Flag, Flags},
	memory::Memory,
};

use self::{instruction::Condition, values::ValueRefI8};

pub struct Cpu {
	pub registers: CPURegisters,
	pub memory: Rc<RefCell<Memory>>,
	pub t_buffer: u32,
}

impl Cpu {
	pub fn new(memory: Rc<RefCell<Memory>>) -> Cpu {
		Cpu {
			memory,
			registers: CPURegisters::new(),
			t_buffer: 0,
		}
	}

	pub fn init(&mut self) {
		// self.registers.pc = 0x100;
		self.registers.pc = 0;
		self.write_16(ValueRefU16::Reg(CPURegister16::AF), 0x01B0);
		self.write_16(ValueRefU16::Reg(CPURegister16::BC), 0x0013);
		self.write_16(ValueRefU16::Reg(CPURegister16::DE), 0x00D8);
		self.write_16(ValueRefU16::Reg(CPURegister16::HL), 0x014D);

		self.registers.sp = 0xFFFE;
		let mut mem = self.memory.borrow_mut();
		mem[0xFF10] = 0x80;
		mem[0xFF11] = 0xBF;
		mem[0xFF12] = 0xF3;
		mem[0xFF14] = 0xBF;
		mem[0xFF16] = 0x3F;
		mem[0xFF19] = 0xBF;
		mem[0xFF1A] = 0x7F;
		mem[0xFF1B] = 0xFF;
		mem[0xFF1C] = 0x9F;
		mem[0xFF1E] = 0xBF;
		mem[0xFF20] = 0xFF;
		mem[0xFF23] = 0xBF;
		mem[0xFF24] = 0x77;
		mem[0xFF25] = 0xF3;
		mem[0xFF26] = 0xF1;
		mem[0xFF40] = 0x91;
		mem[0xFF47] = 0xFC;
		mem[0xFF48] = 0xFF;
		mem[0xFF49] = 0xFF;
	}

	pub fn add_t(&mut self, t: u32) {
		self.t_buffer += t;
	}

	pub fn next_byte(&mut self) -> u8 {
		self.registers.pc = self.registers.pc.wrapping_add(1);
		self.memory.borrow().read(self.registers.pc - 1)
	}

	pub fn next_displacement(&mut self) -> i8 {
		self.next_byte() as i8
	}

	pub fn next_chomp(&mut self) -> u16 {
		self.registers.pc = self.registers.pc.wrapping_add(2);
		self.read_16(ValueRefU16::Mem(self.registers.pc - 2))
	}

	pub fn read_8(&self, value_ref: ValueRefU8) -> u8 {
		match value_ref {
			ValueRefU8::Mem(addr) => self.memory.borrow().read(self.read_16(addr)).to_owned(),
			ValueRefU8::Reg(reg) => self.registers.get_u8(reg),
			ValueRefU8::Raw(x) => x,
		}
	}

	pub fn read_i8(&self, value_ref: ValueRefI8) -> i8 {
		match value_ref {
			ValueRefI8::Mem(i) => self.memory.borrow().read(i) as i8,
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
				as_u16([mem[i], mem[i.wrapping_add(1)]])
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

	pub fn step(&mut self) -> Option<Instruction> {
		if self.t_buffer == 0 {
			let instruction = self.get_next_instruction();
			execute_instruction(instruction.clone(), self);
			self.add_t(1);
			self.t_buffer -= 1;
			return Some(instruction);
		}
		return None;
	}

	// pub fn execute_next_instruction(&mut self) -> Instruction {
	// 	let instruction = self.get_next_instruction().clone();
	// 	execute_instruction(instruction.clone(), self);
	// 	return instruction;
	// }

	pub fn load_cartridge(&mut self, rom: &CartridgeData) {
		let mut mem = self.memory.borrow_mut();
		let (_, data) = rom;

		for i in 0..data.len() {
			mem[i as u16] = data[i];
		}
	}
}
