use gloo::utils::window;
use wasm_bindgen::JsCast;
use web_sys::Gamepad;

use gameboy::joypad::JoypadState;

#[derive(Default)]
pub struct InputState {}

impl InputState {
	fn get_gamepad(&self) -> Option<Gamepad> {
		window()
			.navigator()
			.get_gamepads()
			.ok()?
			.get(0)
			.dyn_into::<Gamepad>()
			.ok()
	}

	pub fn get_controller_state(&self) -> JoypadState {
		let mut state = JoypadState::default();
		if let Some(dom) = window().get("controller_state") {
			if let Some(json) = dom.as_string() {
				if let Ok(dom_state) = serde_json::from_str::<JoypadState>(&json) {
					state |= dom_state;
				}
			}
		}

		if let Some(gp) = self.get_gamepad() {
			state |= gamepad_to_controller_state(&gp);
		}

		state
	}
}

pub fn gamepad_to_controller_state(gp: &Gamepad) -> JoypadState {
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

	JoypadState {
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
