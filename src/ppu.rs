pub mod registers;
use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::Cpu;
use crate::memory::Memory;

enum PPUMode {
	OamScan,
	Draw,
	HBlank,
	VBlank,
}

pub struct Ppu {
	memory: Rc<RefCell<Memory>>,
}

impl Ppu {
	pub fn new(memory: Rc<RefCell<Memory>>) -> Ppu {
		Ppu { memory }
	}
}
