use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CGBState {
	wram_bank: usize,
	vram_bank: usize,
}

impl Default for CGBState {
	fn default() -> Self {
		Self {
			wram_bank: 1,
			vram_bank: 0,
		}
	}
}

impl CGBState {
	// Banking Handlers

	pub fn set_wram_bank(&mut self, bank: u8) {
		self.wram_bank = (bank as usize) & 3;
		self.wram_bank = self.wram_bank.max(1);
	}

	pub fn get_wram_bank(&self) -> usize {
		self.wram_bank
	}

	pub fn set_vram_bank(&mut self, bank: u8) {
		self.vram_bank = (bank as usize) & 1;
	}

	pub fn get_vram_bank(&self) -> usize {
		self.vram_bank
	}
}
