use std::{cell::RefCell, rc::Rc};

pub mod cartridge;
pub mod cpu;
pub mod flags;
pub mod memory;
pub mod memory_registers;
pub mod ppu;
mod state;
pub use state::EmulatorState;

use cpu::Cpu;
use memory::Memory;
use ppu::Ppu;

pub struct Emulator {
	pub memory: Rc<RefCell<Memory>>,
	pub cpu: Cpu,
	pub ppu: Ppu,
}

impl Emulator {
	pub fn new() -> Self {
		let rc = Rc::new(RefCell::new(Memory::new()));

		return Self {
			memory: rc.clone(),
			ppu: Ppu::new(rc.clone()),
			cpu: Cpu::new(rc.clone()).init(),
		};
	}

	pub fn step(&mut self) {
		self.ppu.step();
		self.cpu.step();
	}
}
