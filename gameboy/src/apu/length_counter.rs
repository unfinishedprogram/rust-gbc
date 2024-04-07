use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct LengthCounter {
	enabled: bool,
	length: u8,
	initial: u8,
	frozen: bool,
}

impl Default for LengthCounter {
	fn default() -> Self {
		Self::new(64)
	}
}

impl LengthCounter {
	pub fn new(initial: u8) -> Self {
		Self {
			enabled: true,
			frozen: false,
			length: 0,
			initial,
		}
	}

	pub fn reload(&mut self, length: u8) {
		self.length = (length.wrapping_sub(1)) & self.initial.wrapping_sub(1);
		self.unfreeze();
	}

	pub fn read_length(&self) -> u8 {
		self.length
	}

	pub fn unfreeze(&mut self) {
		self.frozen = false;
	}

	pub fn set_enabled(&mut self, enabled: bool) {
		self.enabled = enabled;
	}

	pub fn enabled(&self) -> bool {
		self.enabled
	}

	// 256hz ticked by the frame-sequencer
	// Only ticks when enabled by NRx4
	// Returns true if the channel should be disabled
	#[must_use]
	pub fn tick(&mut self) -> bool {
		if !self.enabled || self.frozen {
			return false;
		}

		self.length = self.length.wrapping_add(1) & self.initial.wrapping_sub(1);
		if self.length == 0 {
			self.frozen = true;
		}
		self.length == 0
	}
}
