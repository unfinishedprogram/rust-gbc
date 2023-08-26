use crate::{
	ppu::VRAMBank,
	util::bits::{BIT_0, BIT_7},
};
use core::ops::Not;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CGBState {
	wram_bank: usize,
	vram_bank: VRAMBank,
	speed: Speed,
	pub prepare_speed_switch: bool,
}

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug)]
pub enum Speed {
	#[default]
	Normal,
	Double,
}

impl Not for Speed {
	type Output = Speed;
	fn not(self) -> Speed {
		match self {
			Speed::Normal => Speed::Double,
			Speed::Double => Speed::Normal,
		}
	}
}

impl Default for CGBState {
	fn default() -> Self {
		Self {
			prepare_speed_switch: false,
			speed: Speed::Normal,
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

	pub fn write_key1(&mut self, value: u8) {
		self.prepare_speed_switch = value & BIT_0 == BIT_0;
	}

	pub fn read_key1(&self) -> u8 {
		let speed_bit = match self.speed {
			Speed::Normal => 0,
			Speed::Double => BIT_7,
		};

		let switch_bit = if self.prepare_speed_switch { BIT_0 } else { 0 };

		switch_bit | speed_bit
	}

	pub fn perform_speed_switch(&mut self) -> bool {
		if self.prepare_speed_switch {
			self.speed = !self.speed;
			self.prepare_speed_switch = false;
			true
		} else {
			false
		}
	}

	pub fn current_speed(&self) -> Speed {
		self.speed
	}
}
