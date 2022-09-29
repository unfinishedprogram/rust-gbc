use crate::{
	cpu::{instruction::Instruction, Cpu},
	memory::Memory,
	ppu::Ppu,
};
use std::{cell::RefCell, rc::Rc};

pub struct Emulator {
	pub memory: Rc<RefCell<Memory>>,
	pub cpu: Cpu,
	pub ppu: Ppu,
}

impl Emulator {
	pub fn new() -> Self {
		let rc: Rc<RefCell<Memory>> = Rc::new(RefCell::new(Memory::new()));
		let cpu = Cpu::new(rc.clone()).init();
		return Self {
			memory: rc.clone(),
			ppu: Ppu::new(rc.clone()),
			cpu,
		};
	}

	pub fn step(&mut self) -> Option<Instruction> {
		self.ppu.step();
		return self.cpu.step();
	}
}
