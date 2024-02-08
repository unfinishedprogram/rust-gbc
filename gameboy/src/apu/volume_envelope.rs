use serde::{Deserialize, Serialize};

use crate::util::bits::BIT_3;

use super::timer::Timer;

#[derive(Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum Direction {
	Decrease = 0,
	Increase = 1,
}

impl From<Direction> for i8 {
	fn from(val: Direction) -> Self {
		match val {
			Direction::Decrease => -1,
			Direction::Increase => 1,
		}
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VolumeEnvelope {
	pub initial_volume: u8,
	pub direction: Direction,
	pub volume: u8,
	timer: Timer,
}

impl Default for VolumeEnvelope {
	fn default() -> Self {
		Self {
			initial_volume: 0,
			direction: Direction::Decrease,
			volume: 0,
			timer: Timer::new(0),
		}
	}
}

// Envelope Function
impl VolumeEnvelope {
	pub fn read_byte(&self) -> u8 {
		let period = self.timer.get_period() as u8;
		let direction = (self.direction as u8) << 3;
		let initial_volume = self.initial_volume << 4;

		initial_volume | period | direction
	}

	pub fn write_byte(&mut self, value: u8) {
		self.initial_volume = value >> 4;
		self.timer.set_period((value & 0b111) as u16);
		self.direction = if value & BIT_3 == BIT_3 {
			Direction::Increase
		} else {
			Direction::Decrease
		};
		self.reload()
	}

	pub fn tick(&mut self) {
		if self.timer.tick() {
			let new_volume = self.volume.wrapping_add_signed(self.direction.into());
			if new_volume <= 0xF {
				self.volume = new_volume;
			}
		}
	}

	pub fn reload(&mut self) {
		self.volume = self.initial_volume;
		self.timer.reload()
	}
}
