pub struct Flags {
	raw:[bool;4],
}

pub enum Flag {
	Z = 0, N, H, C
}

impl Flags {
	pub fn new() -> Flags {
		Flags {
			raw:[false;4],
		}
	}

	pub fn set(&mut self, flag:Flag) {
		self.raw[flag as usize] = true;
	}

	pub fn clear(&mut self, flag:Flag) {
		self.raw[flag as usize] = false;
	}

	pub fn get(&self, flag:Flag) -> bool{
		self.raw[flag as usize]
	}
}