use super::{
	flags::{self, set_bit_flag, BitFlag},
	EmulatorState,
};

pub trait TimerController {
	fn update_timer(&mut self, cycles: u64);
	fn get_speed(&self) -> u64;
	fn is_enabled(&self) -> bool;
	const DIV: u16 = 0xFF04;
	const TIMA: u16 = 0xFF05;
	const TMA: u16 = 0xFF06;
	const TAC: u16 = 0xFF07;
}

impl TimerController for EmulatorState {
	fn get_speed(&self) -> u64 {
		match self.io_register_state[Self::TAC] & 0b11 {
			0 => 1024,
			1 => 16,
			2 => 64,
			3 => 256,
			_ => unreachable!(),
		}
	}

	fn is_enabled(&self) -> bool {
		self.io_register_state[Self::TAC] & 0b100 == 0b100
	}

	fn update_timer(&mut self, cycles: u64) {
		self.div_clock += cycles;

		if self.is_enabled() {
			self.timer_clock += cycles;
			if self.timer_clock >= self.get_speed() {
				if self.io_register_state[Self::TIMA] == 255 {
					self.io_register_state[Self::TIMA] = self.io_register_state[Self::TMA];
					set_bit_flag(
						self,
						BitFlag::InterruptRequest,
						flags::InterruptFlag::Timer as u8,
					);
				} else {
					self.io_register_state[Self::TIMA] += 1;
				}
				self.timer_clock -= self.get_speed();
			}
		}

		if self.div_clock >= 256 {
			self.div_clock -= 256;
			if self.io_register_state[Self::DIV] == 255 {
				self.io_register_state[Self::DIV] = 0;
			} else {
				self.io_register_state[Self::DIV] += 1;
			}
		}
	}
}