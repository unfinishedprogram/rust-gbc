use super::timer::Timer;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Sweep {
	pub enabled: bool,
	pub negate: bool,
	pub shift: u8,
	pub timer: Timer,
	pub shadow_frequency: u16,
}

impl Sweep {
	pub fn trigger(&mut self, frequency: u16) {
		self.timer.reload();
		self.shadow_frequency = frequency;
		self.enabled = self.shift != 0 || self.timer.get_period() != 0;
		if self.shift != 0 {
			self.calculate();
		}
	}

	// Updates the frequency shadow register
	// Returns true if the channel should be disabled
	fn calculate(&mut self) -> bool {
		let new_freq = self.shadow_frequency >> self.shift;

		if self.negate {
			self.shadow_frequency -= new_freq;
		} else {
			self.shadow_frequency += new_freq;
		}
		self.shadow_frequency > 2047
	}

	fn next_shadow_freq(&self) -> u16 {
		let freq_diff = self.shadow_frequency >> self.shift;

		if self.negate {
			self.shadow_frequency.wrapping_sub(freq_diff)
		} else {
			self.shadow_frequency.wrapping_add(freq_diff)
		}
	}

	// If it returns true, disable the channel
	// The new frequency should be written back to the source
	// Clocked at 128hz by the frame sequencer
	pub fn tick(&mut self) -> (bool, Option<u16>) {
		if !self.enabled {
			return (false, None);
		}

		if self.timer.tick() && self.shift != 0 {
			let new_freq = self.next_shadow_freq();
			if new_freq > 2047 {
				(true, None)
			} else {
				self.shadow_frequency = new_freq;
				let new_new_freq = self.next_shadow_freq();
				let disable_channel = new_new_freq > 2047;
				(disable_channel, Some(new_freq))
			}
		} else {
			(false, None)
		}
	}

	pub fn write_byte(&mut self, value: u8) {
		self.timer.set_period(((value & 0b01110000) >> 4) as u16);
		self.negate = value & 0b1000 != 0;
		self.shift = value & 0b0111;
	}

	pub fn read_byte(&self) -> u8 {
		let period = (self.timer.get_period() << 4) as u8;
		let negate = (self.negate as u8) << 3;
		let shift = self.shift;

		period | negate | shift
	}
}
