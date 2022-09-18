pub mod registers;
use crate::cpu::Cpu;
use crate::util::bitmap::bit_set;

pub trait PPU {
	fn is_active(&self) -> bool;
}

impl PPU for Cpu {
	fn is_active(&self) -> bool {
		bit_set(self.memory[registers::PPURegister::LCDC as usize], 7)
	}
}
