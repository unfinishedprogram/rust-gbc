use super::{flags::INT_TIMER, EmulatorState};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TimerState {
	pub timer_clock: u64,
	pub div_clock: u8,
}

pub trait Timer {
	fn update_timer(&mut self, cycles: u64);
	fn timer_speed(&self) -> u64;
	fn timer_enabled(&self) -> bool;
	const DIV: u16 = 0xFF04;
	const TIMA: u16 = 0xFF05;
	const TMA: u16 = 0xFF06;
	const TAC: u16 = 0xFF07;
}

impl Timer for EmulatorState {
	fn timer_speed(&self) -> u64 {
		match self.io_register_state[Self::TAC] & 0b11 {
			0 => 1024,
			1 => 16,
			2 => 64,
			3 => 256,
			_ => unreachable!(),
		}
	}

	fn timer_enabled(&self) -> bool {
		self.io_register_state[Self::TAC] & 0b100 == 0b100
	}

	fn update_timer(&mut self, cycles: u64) {
		self.timer_state.div_clock = self.timer_state.div_clock.wrapping_add(cycles as u8);

		if self.timer_enabled() {
			self.timer_state.timer_clock += cycles;
			if self.timer_state.timer_clock >= self.timer_speed() {
				let (next_tima, overflow) = self.io_register_state[Self::TIMA].overflowing_add(1);

				if overflow {
					self.io_register_state[Self::TIMA] = self.io_register_state[Self::TMA];
					self.request_interrupt(INT_TIMER);
				} else {
					self.io_register_state[Self::TIMA] = next_tima;
				}
				self.timer_state.timer_clock -= self.timer_speed();
			}
		}

		let (timer, overflow) = self.timer_state.div_clock.overflowing_add(cycles as u8);
		self.timer_state.div_clock = timer;
		if overflow {
			self.io_register_state[Self::DIV] = self.io_register_state[Self::DIV].wrapping_add(1);
		}
	}
}
