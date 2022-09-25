pub mod flags;
pub mod interrupts;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lcd::Lcd;
use crate::memory::Memory;

use crate::memory_registers::MemoryRegister::*;

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
		let mut mem = self.memory.borrow_mut();
		mem[LCDC as u16] = (mem[LCDC as u16] & 0b11111100) | mode as u8;
	}

	pub fn get_mode(&mut self) -> PPUMode {
		let mut mem = self.memory.borrow_mut();
		let num = mem[LCDC as u16] & 0b11111100;
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
			match self.get_mode() {
				PPUMode::HBlank => {
					self.set_mode(PPUMode::OamScan);
					self.t_state += 80;
				}
				PPUMode::VBlank => {
					if self.get_ly() == 153 {
						self.set_mode(PPUMode::OamScan);
						self.set_ly(0);
						self.t_state += 80;
					} else {
						self.set_ly(self.get_ly() + 1);
						self.t_state += 456;
					}
				}
				PPUMode::OamScan => {
					self.set_mode(PPUMode::Draw);
					self.t_state += 172;
				}
				PPUMode::Draw => {
					self.set_mode(PPUMode::HBlank);
					self.t_state += 204;
				}
			}
		}
		self.t_state -= 1;
	}

	// fn set_mode(&mut self, mode: PPUMode) {
	// 	let mut mem = self.memory.borrow_mut();
	// 	let new_val = (mem[STAT as u16] & 0b11111100) & (mode as u8);
	// 	mem[STAT as u16] = new_val;
	// }
}
