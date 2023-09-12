use crate::{cgb::Speed, util::bits::BIT_2};

use serde::{Deserialize, Serialize};
use sm83::flags::interrupt::TIMER;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Timer {
	system_clock: u16,

	div: u8,
	tima: u8,
	tma: u8,
	tac: u8,

	enabled: bool,
	pub tima_delay: u8,
}

impl Timer {
	pub fn set_div(&mut self, _: u8, speed: Speed) {
		// Detect falling edge for timer
		if self.enabled {
			let bit = match speed {
				Speed::Normal => self.timer_speed(),
				Speed::Double => self.timer_speed() << 1,
			} >> 1;
			if (self.system_clock & bit) == bit {
				self.increment_tima();
			}
		}

		self.div = 0;
		self.system_clock = 0;
	}

	pub fn set_tima(&mut self, value: u8) {
		self.tima = value;
	}

	pub fn set_tma(&mut self, value: u8) {
		self.tma = value;
	}

	pub fn set_tac(&mut self, value: u8) {
		self.tac = value;
		self.enabled = self.tac & BIT_2 == BIT_2
	}

	pub fn get_div(&self) -> u8 {
		self.div
	}
	pub fn get_tima(&self) -> u8 {
		self.tima
	}
	pub fn get_tma(&self) -> u8 {
		self.tma
	}
	pub fn get_tac(&self) -> u8 {
		self.tac
	}

	fn timer_speed(&self) -> u16 {
		match self.tac & 0b11 {
			0 => 1024,
			1 => 16,
			2 => 64,
			3 => 256,
			_ => unreachable!(),
		}
	}

	fn step_cycle(&mut self, speed: Speed, interrupt_request: &mut u8) {
		let from = self.system_clock;
		self.system_clock = self.system_clock.wrapping_add(1);
		let to = self.system_clock;

		// Div is the top 8 bits of the 16 bit system clock
		self.div = (self.system_clock >> 8) as u8;

		// Detect falling edge for timer
		if self.enabled {
			let bit = match speed {
				Speed::Normal => self.timer_speed(),
				Speed::Double => self.timer_speed() << 1,
			};

			if (from & bit) != (to & bit) {
				self.increment_tima();
			}
		}

		if self.tima_delay > 0 {
			self.tima_delay -= 1;
			if self.tima_delay == 0 {
				self.tima = self.tma;
				*interrupt_request |= TIMER;
			}
		}
	}

	fn increment_tima(&mut self) {
		self.tima = self.tima.wrapping_add(1);
		if self.tima == 0 {
			self.tima_delay = 5;
		}
	}

	pub fn step(&mut self, cycles: u64, speed: Speed, interrupt_request: &mut u8) {
		for _ in 0..cycles * 4 {
			self.step_cycle(speed, interrupt_request);
		}
	}
}
