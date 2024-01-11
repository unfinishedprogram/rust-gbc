pub struct Timer {
	counter: u32,
	period: u32,
}

impl Timer {
	pub fn tick(&mut self) -> bool {
		self.counter -= 1;
		if self.counter == 0 {
			self.counter = self.period;
			true
		} else {
			false
		}
	}
}
