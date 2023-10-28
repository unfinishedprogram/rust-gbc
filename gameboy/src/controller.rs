use std::ops::BitOrAssign;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct ControllerState {
	pub a: bool,
	pub b: bool,
	pub select: bool,
	pub start: bool,
	pub right: bool,
	pub left: bool,
	pub up: bool,
	pub down: bool,
}

impl BitOrAssign for ControllerState {
	fn bitor_assign(&mut self, rhs: Self) {
		self.a |= rhs.a;
		self.b |= rhs.b;
		self.select |= rhs.select;
		self.start |= rhs.start;
		self.right |= rhs.right;
		self.left |= rhs.left;
		self.up |= rhs.up;
		self.down |= rhs.down;
	}
}

impl ControllerState {
	pub fn as_byte(&self) -> u8 {
		![
			self.a,
			self.b,
			self.select,
			self.start,
			self.right,
			self.left,
			self.up,
			self.down,
		]
		.iter()
		.enumerate()
		.filter(|(_, &pressed)| pressed)
		.map(|(index, _)| 1 << index)
		.sum::<u8>()
	}
}
