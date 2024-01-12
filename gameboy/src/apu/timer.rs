use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Timer {
	pub counter: u16,
	pub period: u16,
}

impl Timer {
	pub fn new(period: u16) -> Self {
		Self {
			counter: period,
			period,
		}
	}

	pub fn tick(&mut self) -> bool {
		self.counter -= 1;
		if self.counter == 0 {
			self.reload();
			true
		} else {
			false
		}
	}

	pub fn reload(&mut self) {
		self.counter = self.period;
	}
}
