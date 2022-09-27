use std::cell::RefCell;
use std::rc::Rc;

use crate::flags;
use crate::flags::{get_bit_flag, set_bit_flag, set_bit_flag_to, BitFlag, STATFlag};
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

	pub fn get_mode(&self) -> PPUMode {
		let mem = self.memory.borrow();
		let num = mem[STAT as u16] & 0b00000011;
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
		let mut mem = self.memory.borrow_mut();
		mem[LY as u16] = value;
		let lyc_status = mem[LYC as u16] == value;
		set_bit_flag_to(&mut mem, BitFlag::Stat(STATFlag::LYCeqLY), lyc_status);

		if lyc_status && get_bit_flag(&mem, BitFlag::Stat(STATFlag::LYCeqLUInterruptEnable)) {
			set_bit_flag(
				&mut mem,
				BitFlag::InterruptRequest(flags::InterruptFlag::LcdStat),
			);
		}
	}

	pub fn set_mode(&mut self, mode: PPUMode) {
		use STATFlag::*;
		match mode {
			PPUMode::HBlank => {
				let mut mem = self.memory.borrow_mut();
				if get_bit_flag(&mem, BitFlag::Stat(HBlankStatInterruptEnable)) {
					set_bit_flag(
						&mut mem,
						BitFlag::InterruptRequest(flags::InterruptFlag::LcdStat),
					);
				}
				self.t_state += 204;
			}
			PPUMode::VBlank => {
				{
					let mut mem = self.memory.borrow_mut();
					if get_bit_flag(&mem, BitFlag::Stat(VBlankStatInterruptEnable)) {
						set_bit_flag(
							&mut mem,
							BitFlag::InterruptRequest(flags::InterruptFlag::LcdStat),
						);
					}
				}
				self.t_state += 456;
				self.set_ly(self.get_ly() + 1)
			}

			PPUMode::OamScan => {
				{
					let mut mem = self.memory.borrow_mut();
					if get_bit_flag(&mem, BitFlag::Stat(OAMStatInterruptEnable)) {
						set_bit_flag(
							&mut mem,
							BitFlag::InterruptRequest(flags::InterruptFlag::LcdStat),
						);
					}
				}

				self.t_state += 80;
				if self.get_ly() >= 153 {
					self.set_ly(0);
				} else {
					self.set_ly(self.get_ly() + 1);
				}
			}
			PPUMode::Draw => self.t_state += 172,
		}
		let mut mem = self.memory.borrow_mut();

		mem[STAT as u16] = (mem[STAT as u16] & 0b11111100) | mode as u8;
	}

	pub fn step(&mut self) {
		if self.t_state == 0 {
			use PPUMode::*;
			match (self.get_mode(), self.get_ly()) {
				(OamScan, _) => self.set_mode(Draw),
				(Draw, _) => self.set_mode(HBlank),
				(HBlank, 0..=143) => self.set_mode(OamScan),
				(HBlank, _) => self.set_mode(VBlank),
				(VBlank, 144..=153) => self.set_mode(VBlank),
				(VBlank, _) => self.set_mode(OamScan),
			}
		}
		self.t_state -= 1;
	}
}
