pub mod flags;
pub mod interrupts;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lcd::Lcd;
use crate::memory::Memory;
// use registers::PPURegister::*;

enum PPUMode {
	HBlank = 0,
	VBlank = 1,
	OamScan = 2,
	Draw = 3,
}

pub struct Ppu {
	memory: Rc<RefCell<Memory>>,
}

impl Ppu {
	pub fn new(memory: Rc<RefCell<Memory>>) -> Ppu {
		Ppu { memory }
	}

	pub fn step(&mut self, lcd: &mut Lcd) {
		let mem = self.memory.borrow_mut();
	}

	// fn set_mode(&mut self, mode: PPUMode) {
	// 	let mut mem = self.memory.borrow_mut();
	// 	let new_val = (mem[STAT as u16] & 0b11111100) & (mode as u8);
	// 	mem[STAT as u16] = new_val;
	// }
}
