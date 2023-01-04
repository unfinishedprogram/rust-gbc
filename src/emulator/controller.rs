use std::ops::AddAssign;

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::Gamepad;

#[derive(Deserialize, Serialize)]
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

impl AddAssign for ControllerState {
	fn add_assign(&mut self, rhs: Self) {
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
	pub fn from_gamepad(gp: &Gamepad) -> Self {
		let buttons: Vec<bool> = gp
			.buttons()
			.iter()
			.map(|button| {
				button
					.dyn_into::<web_sys::GamepadButton>()
					.unwrap()
					.pressed()
			})
			.collect();

		let axis: Vec<f64> = gp.axes().iter().map(|v| v.as_f64().unwrap()).collect();

		ControllerState {
			a: buttons[0] || buttons[3],
			b: buttons[2] || buttons[1],
			select: buttons[8],
			start: buttons[9],
			right: axis[0] > 0.25 || *buttons.get(15).unwrap_or(&false), //up
			left: axis[0] < -0.25 || *buttons.get(14).unwrap_or(&false), // RIGHT
			up: axis[1] < -0.25 || *buttons.get(12).unwrap_or(&false),
			down: axis[1] > 0.25 || *buttons.get(13).unwrap_or(&false), // LEFT
		}
	}

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