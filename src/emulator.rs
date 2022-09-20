use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::ppu::Ppu;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Emulator {
	pub memory: Rc<RefCell<Memory>>,
	pub cpu: Cpu,
	pub ppu: Ppu,
}

impl Emulator {
	pub fn new() -> Self {
		let rc: Rc<RefCell<Memory>> = Rc::new(RefCell::new(Memory::new()));
		return Self {
			memory: rc.clone(),
			ppu: Ppu::new(rc.clone()),
			cpu: Cpu::new(rc.clone()),
		};
	}
	pub fn current_t(&self) -> u32 {
		self.memory.borrow().t_state.borrow().to_owned()
	}
}
