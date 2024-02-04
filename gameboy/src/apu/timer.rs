use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Timer {
	counter: u16,
	period: u16,
}

impl Timer {
	pub fn new(period: u16) -> Self {
		Self {
			counter: period,
			period,
		}
	}

	pub fn tick(&mut self) -> bool {
		if self.period == 0 {
			return false;
		}

		if self.counter == 0 {
			self.reload();
			return true;
		}

		self.counter -= 1;
		false

		// let overflow;
		// (self.counter, overflow) = self.counter.overflowing_sub(1);
		// if self.counter == 0 || overflow {
		// 	self.reload();
		// 	true
		// } else {
		// 	false
		// }
	}

	pub fn set_period(&mut self, period: u16) {
		self.period = period;
		self.reload();
	}

	pub fn get_period(&self) -> u16 {
		self.period
	}

	pub fn reload(&mut self) {
		self.counter = self.period;
	}
}
