use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct LengthCounter {
	pub enabled: bool,
	length: u8,
	initial: u8,
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

	pub fn reload(&mut self, length: u8) {
		self.length = length % self.initial;
	}

	pub fn read_length(&self) -> u8 {
		self.length
	}

	// 256hz ticked by the frame-sequencer
	// Only ticks when enabled by NRx4
	// Returns true if the channel should be disabled
	#[must_use]
	pub fn tick(&mut self) -> bool {
		if !self.enabled {
			return false;
		}

		self.length = self.length.wrapping_add(1) & (self.initial - 1);
		if self.length == 0 {
			self.enabled = false;
			true
		} else {
			false
		}
	}
}
