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

	pub tima_delay: u8,

	pub interrupt_requests: u8,
}

impl Timer {
	pub fn set_div(&mut self, _: u8) {
		// Detect falling edge for timer
		if self.timer_enabled() {
			let bit = self.timer_speed() as u16 >> 1;
			if (self.system_clock & bit) == bit {
				self.increment_tima();
			}
		}

		self.div = 0;
		self.system_clock = 0;
	}

	pub fn set_tima(&mut self, value: u8) {
		// if self.tima_delay == 0 {
		self.tima = value;
		// }
	}

	pub fn set_tma(&mut self, value: u8) {
		self.tma = value;
	}

	pub fn set_tac(&mut self, value: u8) {
		self.tac = value;
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

	fn timer_speed(&self) -> u64 {
		match self.tac & 0b11 {
			0 => 1024,
			1 => 16,
			2 => 64,
			3 => 256,
			_ => unreachable!(),
		}
	}

	fn timer_enabled(&self) -> bool {
		self.tac & BIT_2 == BIT_2
	}

	pub fn step_cycle(&mut self, speed: Speed) {
		let from = self.system_clock;

		self.system_clock = self.system_clock.wrapping_add(1);

		let to = self.system_clock;

		// Div is the top 8 bits of the 16 bit system clock
		self.div = (self.system_clock >> 8) as u8;

		// Detect falling edge for timer
		if self.timer_enabled() {
			// let bit = self.timer_speed() as u16;
			let bit = match speed {
				Speed::Normal => self.timer_speed() as u16,
				Speed::Double => (self.timer_speed() as u16) << 1,
			};

			let timer_increment = (from & bit) != (to & bit);
			if timer_increment {
				self.increment_tima();
			}
		}

		if self.tima_delay > 0 {
			self.tima_delay -= 1;
			if self.tima_delay == 0 {
				self.tima = self.tma;
				self.interrupt_requests |= TIMER;
			}
		}
	}

	fn increment_tima(&mut self) {
		let (next_tima, overflow) = self.tima.overflowing_add(1);

		if overflow {
			self.tima = 0;
			self.tima_delay = 5;
		} else {
			self.tima = next_tima;
		}
	}

	pub fn step(&mut self, cycles: u64, speed: Speed) {
		let cycles = match speed {
			Speed::Normal => cycles * 4,
			Speed::Double => cycles * 8,
		};

		for _ in 0..cycles {
			self.step_cycle(speed);
		}
	}
}
