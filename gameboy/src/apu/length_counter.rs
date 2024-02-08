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

	// Enables and loads length
	pub fn reload(&mut self, length: u8) {
		self.length = self.initial - length;
		// self.enabled = true;
		log::error!("Writing to counter length: {:}", self.initial - length);
	}

	pub fn read_length(&self) -> u8 {
		self.initial - self.length
	}

	// 256hz ticked by the frame-sequencer
	// Only ticks when enabled by NRx4
	// Returns true if the channel should be disabled
	#[must_use]
	pub fn tick(&mut self) -> bool {
		if !self.enabled {
			return false;
		}

		if self.length - self.initial == 0 {
			return true;
		}
		self.length += 1;
		if self.length - self.initial == 0 {
			self.enabled = false;
			log::error!("Disabling channel");
			true
		} else {
			false
		}
	}
}
