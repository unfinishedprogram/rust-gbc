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
