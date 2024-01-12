use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct LengthCounter {
	pub enabled: bool,
	pub length: u8,
}

impl LengthCounter {
	// Enables and loads length
	pub fn reload(&mut self, length: u8) {
		self.enabled = true;
		self.length = length;
	}

	// 256hz ticked by the frame-sequencer
	// Only ticks when enabled by NRx4
	// Returns true if the channel should be disabled
	pub fn tick(&mut self) -> bool {
		if !self.enabled {
			return false;
		}

		if self.length > 0 {
			self.length -= 1;
			if self.length == 0 {
				self.enabled = false;
				return true;
			}
		}

		false
	}
}
