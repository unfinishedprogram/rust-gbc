use std::collections::{hash_map::RandomState, HashSet};

use egui::Key;
use wasm_bindgen::JsCast;
use web_sys::Gamepad;

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

impl From<&HashSet<Key, RandomState>> for ControllerState {
	fn from(keys: &HashSet<Key, RandomState>) -> Self {
		use Key::*;
		Self {
			a: keys.contains(&Z),
			b: keys.contains(&X),
			select: keys.contains(&Space),
			start: keys.contains(&Enter),
			right: keys.contains(&ArrowRight),
			left: keys.contains(&ArrowLeft),
			up: keys.contains(&ArrowUp),
			down: keys.contains(&ArrowDown),
		}
	}
}

impl From<&Gamepad> for ControllerState {
	fn from(gp: &Gamepad) -> Self {
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
			a: buttons[0],
			b: buttons[1],
			select: buttons[8],
			start: buttons[9],
			right: axis[0] > 0.1 || *buttons.get(15).unwrap_or(&false), //up
			left: axis[0] < -0.1 || *buttons.get(14).unwrap_or(&false), // RIGHT
			up: axis[1] < -0.1 || *buttons.get(12).unwrap_or(&false),
			down: axis[1] > 0.1 || *buttons.get(13).unwrap_or(&false), // LEFT
		}
	}
}
