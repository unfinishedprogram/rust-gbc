use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct LengthCounter {
	pub enabled: bool,
	pub length: u8,
	pub initial: u8,
}

impl Default for LengthCounter {
	fn default() -> Self {
		Self::new(64)
	}
}

impl LengthCounter {
	pub fn new(initial: u8) -> Self {
		Self {
			enabled: false,
			length: 0,
			initial,
		}
	}

	// Enables and loads length
	pub fn reload(&mut self, length: u8) {
		self.length = self.initial - length;
	}

	// 256hz ticked by the frame-sequencer
	// Only ticks when enabled by NRx4
	// Returns true if the channel should be disabled
	pub fn tick(&mut self) -> bool {
		if self.length == 0 || !self.enabled {
			return false;
		}

		self.length -= 1;
		if self.length == 0 {
			self.enabled = false;
			true
		} else {
			false
		}
	}
}
