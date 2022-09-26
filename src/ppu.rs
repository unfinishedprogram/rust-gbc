use std::cell::RefCell;
use std::rc::Rc;

use crate::flags::set_bit_flag;
use crate::memory::Memory;

use crate::memory_registers::MemoryRegister::*;

#[derive(Debug)]
pub enum PPUMode {
	HBlank = 0,
	VBlank = 1,
	OamScan = 2,
	Draw = 3,
}

pub struct Ppu {
	memory: Rc<RefCell<Memory>>,
	t_state: u32,
}

impl Ppu {
	pub fn new(memory: Rc<RefCell<Memory>>) -> Ppu {
		Ppu { memory, t_state: 0 }
	}

	pub fn set_mode(&mut self, mode: PPUMode) {
		match mode {
			PPUMode::HBlank => self.t_state += 204,
			PPUMode::VBlank => {
				self.t_state += 456;
				self.set_ly(self.get_ly() + 1)
			}
			PPUMode::OamScan => {
				self.t_state += 80;
				self.set_ly((self.get_ly() + 1) % 153)
			}
			PPUMode::Draw => self.t_state += 172,
		}
		let mut mem = self.memory.borrow_mut();
		mem[LCDC as u16] = (mem[LCDC as u16] & 0b11111100) | mode as u8;
	}

	pub fn get_mode(&self) -> PPUMode {
		let mem = self.memory.borrow();
		let num = mem[LCDC as u16] & 0b00000011;
		return match num {
			0 => PPUMode::HBlank,
			1 => PPUMode::VBlank,
			2 => PPUMode::OamScan,
			3 => PPUMode::Draw,
			_ => PPUMode::HBlank,
		};
	}

	pub fn get_ly(&self) -> u8 {
		return self.memory.borrow()[LY as u16];
	}
	pub fn set_ly(&mut self, value: u8) {
		self.memory.borrow_mut()[LY as u16] = value;
	}

	pub fn step(&mut self) {
		if self.t_state == 0 {
			use PPUMode::*;
			match (self.get_mode(), self.get_ly()) {
				(OamScan, _) => self.set_mode(Draw),
				(Draw, _) => self.set_mode(HBlank),
				(HBlank, 0..=143) => self.set_mode(OamScan),
				(HBlank, 144..=u8::MAX) => self.set_mode(VBlank),
				(VBlank, 144..=152) => self.set_mode(VBlank),
				(VBlank, 153) => self.set_mode(OamScan),
				_ => self.set_mode(VBlank),
			}
		}
		self.t_state -= 1;
	}
}
