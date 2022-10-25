pub mod flags;
mod gb_stack;
pub mod instruction;
pub mod registers;
mod state;
pub mod values;
pub use state::CPUState;

use std::{cell::RefCell, rc::Rc};

use super::flags::*;
use super::memory::Memory;
use instruction::{
	execute::execute_instruction, fetch::fetch_instruction, opcode::Opcode, Instruction,
};

use registers::{CPURegister16, CPURegisters};
use values::{ValueRefU16, ValueRefU8};

use self::flags::{Flag, Flags};
use self::{instruction::condition::Condition, values::ValueRefI8};

pub struct Cpu {
	pub registers: CPURegisters,
	pub memory: Rc<RefCell<Memory>>,
	pub t_buffer: u32,
	pub interrupt_enable: bool,
	interrupt_next_state: Option<bool>,
}

impl Cpu {
	pub fn new(memory: Rc<RefCell<Memory>>) -> Cpu {
		Cpu {
			memory,
			registers: CPURegisters::default(),
			t_buffer: 0,
			interrupt_next_state: None,
			interrupt_enable: false,
		}
	}

	pub fn disable_interrupts(&mut self) {
		_ = self.interrupt_next_state.insert(false);
	}

	pub fn enable_interrupts(&mut self) {
		_ = self.interrupt_next_state.insert(true);
	}

	pub fn init(mut self) -> Self {
		self.registers.pc = 0x100;
		self.registers.sp = 0xFFFE;

		self.write_16(ValueRefU16::Reg(CPURegister16::AF), 0x01B0);
		self.write_16(ValueRefU16::Reg(CPURegister16::BC), 0x0013);
		self.write_16(ValueRefU16::Reg(CPURegister16::DE), 0x00D8);
		self.write_16(ValueRefU16::Reg(CPURegister16::HL), 0x014D);

		{
			let mut mem = self.memory.borrow_mut();
			mem.write(0xFF10, 0x80);
			mem.write(0xFF11, 0xBF);
			mem.write(0xFF12, 0xF3);
			mem.write(0xFF14, 0xBF);
			mem.write(0xFF16, 0x3F);
			mem.write(0xFF19, 0xBF);
			mem.write(0xFF1A, 0x7F);
			mem.write(0xFF1B, 0xFF);
			mem.write(0xFF1C, 0x9F);
			mem.write(0xFF1E, 0xBF);
			mem.write(0xFF20, 0xFF);
			mem.write(0xFF23, 0xBF);
			mem.write(0xFF24, 0x77);
			mem.write(0xFF25, 0xF3);
			mem.write(0xFF26, 0xF1);
			mem.write(0xFF40, 0x91);
			mem.write(0xFF47, 0xFC);
			mem.write(0xFF48, 0xFF);
			mem.write(0xFF49, 0xFF);
		}

		self
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

	pub fn read_8(&mut self, value_ref: ValueRefU8) -> u8 {
		match value_ref {
			ValueRefU8::Mem(addr) => {
				self.add_t(2);
				let index = self.read_16(addr);
				self.memory.borrow().read(index)
			}
			ValueRefU8::Reg(reg) => self.registers[reg],
			ValueRefU8::Raw(x) => x,
		}
	}

	pub fn read_i8(&mut self, value_ref: ValueRefI8) -> i8 {
		match value_ref {
			ValueRefI8::Mem(addr) => {
				self.add_t(2);
				self.memory.borrow().read(addr) as i8
			}
			ValueRefI8::Reg(reg) => self.registers[reg] as i8,
			ValueRefI8::Raw(x) => x,
		}
	}

	pub fn write_8(&mut self, value_ref: ValueRefU8, value: u8) {
		match value_ref {
			ValueRefU8::Mem(addr) => {
				self.add_t(2);
				let index = self.read_16(addr);
				self.memory.borrow_mut().write(index, value);
			}
			ValueRefU8::Reg(reg) => self.registers[reg] = value,
			ValueRefU8::Raw(_) => unreachable!(),
		}
	}

	pub fn read_16(&mut self, value_ref: ValueRefU16) -> u16 {
		match value_ref {
			ValueRefU16::Mem(i) => {
				self.add_t(2);
				let mem = self.memory.borrow();
				u16::from_le_bytes([mem.read(i), mem.read(i.wrapping_add(1))])
			}
			ValueRefU16::Reg(reg) => self.registers.get_u16(reg),
			ValueRefU16::Raw(x) => x,
		}
	}

	pub fn write_16(&mut self, value_ref: ValueRefU16, value: u16) {
		match value_ref {
			ValueRefU16::Mem(i) => {
				let mut mem = self.memory.borrow_mut();
				mem.write(i + 1, (value >> 8) as u8);
				mem.write(i, (value & 0xFF) as u8);
			}
			ValueRefU16::Reg(reg) => self.registers.set_u16(reg, value),
			ValueRefU16::Raw(_) => unreachable!(),
		}
	}

	pub fn get_next_instruction(&mut self) -> Instruction {
		let opcode: Opcode = Opcode::from(self.next_byte());
		fetch_instruction(self, opcode)
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

	fn check_interrupt(&self, interrupt: InterruptFlag) -> bool {
		let mem = self.memory.borrow();

		get_bit_flag(&mem, BitFlag::InterruptEnable(interrupt))
			&& get_bit_flag(&mem, BitFlag::InterruptRequest(interrupt))
	}

	fn clear_request(&mut self, interrupt: InterruptFlag) {
		clear_bit_flag(
			&mut self.memory.borrow_mut(),
			BitFlag::InterruptRequest(interrupt),
		);
	}

	fn get_interrupt(&mut self) -> Option<Instruction> {
		use InterruptFlag::*;
		let to_check = [VBlank, LcdStat, Timer, Serial, JoyPad];
		if self.interrupt_enable {
			for interrupt in to_check {
				if self.check_interrupt(interrupt) {
					self.clear_request(interrupt);
					return Some(Instruction::INT(interrupt));
				}
			}
		}
		return None;
	}

	pub fn get_next_instruction_or_interrupt(&mut self) -> Instruction {
		return match self.get_interrupt() {
			Some(inst) => {
				self.disable_interrupts();
				inst
			}
			None => self.get_next_instruction(),
		};
	}

	pub fn step(&mut self) -> Option<Instruction> {
		if self.t_buffer == 0 {
			if let Some(interrupt_enable) = self.interrupt_next_state {
				self.interrupt_next_state = None;
				self.interrupt_enable = interrupt_enable;
			}
			let instruction = self.get_next_instruction_or_interrupt();
			execute_instruction(instruction.clone(), self);
			self.add_t(1);

			return Some(instruction);
		}
		self.t_buffer -= 1;
		return None;
	}
}
