use serde::{Deserialize, Serialize};

use crate::ppu::VRAMBank;

#[derive(Clone, Serialize, Deserialize)]
pub struct CGBState {
	wram_bank: usize,
	vram_bank: VRAMBank,
}

impl Default for CGBState {
	fn default() -> Self {
		Self {
			wram_bank: 1,
			vram_bank: VRAMBank::Bank0,
		}
	}
}

impl CGBState {
	pub fn set_vram_bank(&mut self, bank: u8) {
		self.vram_bank = if (bank) & 1 == 1 {
			VRAMBank::Bank1
		} else {
			VRAMBank::Bank0
		};
	}

	pub fn get_vram_bank(&self) -> VRAMBank {
		self.vram_bank
	}
}
