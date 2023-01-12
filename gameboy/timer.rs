use crate::util::bits::BIT_2;

use super::flags::INT_TIMER;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Timer {
	timer_clock: u64,
	div_clock: u8,

	div: u8,
	tima: u8,
	tma: u8,
	tac: u8,

	pub interrupt_requests: u8,
}

impl Timer {
	pub fn set_div(&mut self, _: u8) {
		self.div = 0;
		self.tima = self.tma;
		self.div_clock = 0;
	}

	pub fn set_tima(&mut self, value: u8) {
		self.tima = value;
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

	pub fn step(&mut self, cycles: u64) {
		if self.timer_enabled() {
			self.timer_clock += cycles;
			if self.timer_clock >= self.timer_speed() {
				let (next_tima, overflow) = self.tima.overflowing_add(1);

				if overflow {
					self.tima = self.tma;
					self.interrupt_requests |= INT_TIMER;
				} else {
					self.tima = next_tima;
				}
				self.timer_clock -= self.timer_speed();
			}
		}

		let (timer, overflow) = self.div_clock.overflowing_add(cycles as u8);
		self.div_clock = timer;

		if overflow {
			self.div = self.div.wrapping_add(1);
		}
	}
}
