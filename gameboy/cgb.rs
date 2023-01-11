use serde::{Deserialize, Serialize};

use crate::util::bits::BIT_7;

/// Handles reading and writing of color pallette data for CGB mode
#[derive(Clone, Serialize, Deserialize)]
pub struct ColorRamController {
	data: Vec<u8>,
	index: usize,
	increment: bool,
}

impl Default for ColorRamController {
	fn default() -> Self {
		Self {
			data: vec![0; 64],
			index: 0,
			increment: false,
		}
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CGBState {
	wram_bank: usize,
	vram_bank: usize,

	pub bg_color: ColorRamController,
	pub obj_color: ColorRamController,
}

impl Default for CGBState {
	fn default() -> Self {
		Self {
			wram_bank: 1,
			vram_bank: 0,
			bg_color: ColorRamController::default(),
			obj_color: ColorRamController::default(),
		}
	}
}

impl ColorRamController {
	pub fn write_spec(&mut self, value: u8) {
		self.increment = value & BIT_7 == BIT_7;
		self.index = value as usize & 0b00111111;
	}

	pub fn read_spec(&self) -> u8 {
		let increment = if self.increment { BIT_7 } else { 0 };
		let index = self.index;
		increment | index as u8
	}

	pub fn read_data(&self) -> u8 {
		self.data[self.index]
	}

	pub fn write_data(&mut self, value: u8) {
		self.data[self.index] = value;
		// Only increment on writes, not reads
		if self.increment {
			self.index += 1;
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
