use crate::util::bits::BIT_2;

use serde::{Deserialize, Serialize};
use sm83::Interrupt;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Timer {
	system_clock: u16,
	tima: u8,
	tma: u8,
	tac: u8,
	tima_delay: u8,
}

impl Timer {
	pub fn set_div(&mut self, _: u8) {
		// Detect falling edge for timer
		if self.current_output_clock() {
			self.increment_tima();
		}
		self.system_clock = 0;
	}

	fn current_output_clock(&self) -> bool {
		let bit = self.tac_freq() >> 1;
		(self.system_clock & bit) == bit
	}

	pub fn write_tima(&mut self, value: u8) {
		self.tima = value;
	}

	pub fn write_tma(&mut self, value: u8) {
		self.tma = value;
	}

	pub fn set_tac(&mut self, value: u8) {
		let tac_enable = value & BIT_2 != 0;
		if tac_enable & !self.tac_enabled() && self.current_output_clock() {
			self.increment_tima();
		}

		self.tac = value;
	}

	fn tac_enabled(&self) -> bool {
		self.tac & BIT_2 != 0
	}

	pub fn get_div(&self) -> u8 {
		(self.system_clock >> 8) as u8
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

	fn tac_freq(&self) -> u16 {
		match self.tac & 0b11 {
			0 => 1024,
			1 => 16,
			2 => 64,
			3 => 256,
			_ => unreachable!(),
		}
	}

	fn increment_tima(&mut self) {
		self.tima = self.tima.wrapping_add(1);
		if self.tima == 0 {
			self.tima_delay = 4;
		}
	}

	pub fn step(&mut self, interrupt_request: &mut u8) {
		let from = self.system_clock;
		self.system_clock = self.system_clock.wrapping_add(1);
		let to = self.system_clock;

		let bit = self.tac_freq() >> 1;
		if from & bit != 0 && to & bit == 0 && self.tac_enabled() {
			self.increment_tima();
		}

		if self.tima_delay > 0 {
			self.tima_delay -= 1;
			if self.tima_delay == 0 {
				self.tima = self.tma;
				*interrupt_request |= Interrupt::Timer.flag_bit();
			}
		}
	}
}
