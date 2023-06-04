use std::{cell::RefCell, collections::HashSet, rc::Rc};

use gloo::{
	events::EventListener,
	utils::{document_element, window},
};
use wasm_bindgen::JsCast;
use web_sys::Gamepad;

use gameboy::controller::ControllerState;

#[derive(Default)]
struct InputStateInner {
	keys: HashSet<String>,
}

pub struct InputState {
	inner: Rc<RefCell<InputStateInner>>,
	_key_down: EventListener,
	_key_up: EventListener,
}

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

	pub fn get_controller_state(&self) -> ControllerState {
		let mut state = {
			let keys = &self.inner.borrow().keys;
			ControllerState {
				a: keys.contains("z"),
				b: keys.contains("x"),
				select: keys.contains("Tab"),
				start: keys.contains("Enter"),
				right: keys.contains("ArrowRight"),
				left: keys.contains("ArrowLeft"),
				up: keys.contains("ArrowUp"),
				down: keys.contains("ArrowDown"),
			}
		};

		if let Some(dom) = window().get("controller_state") {
			if let Some(json) = dom.as_string() {
				if let Ok(dom_state) = serde_json::from_str::<ControllerState>(&json) {
					state += dom_state;
				}
			}
		}

		if let Some(gp) = self.get_gamepad() {
			state += gamepad_to_controller_state(&gp);
		}

		state
	}
}

impl Default for InputState {
    fn default() -> Self {
        let inner = Rc::new(RefCell::new(InputStateInner::default()));

		let key_down = {
			let inner = inner.clone();
			EventListener::new(&document_element(), "keydown", move |e| {
				let event = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
				inner.borrow_mut().keys.insert(event.key());
			})
		};

		let key_up = {
			let inner = inner.clone();
			EventListener::new(&document_element(), "keyup", move |e| {
				let event = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
				inner.borrow_mut().keys.remove(&event.key());
			})
		};

		Self {
			_key_down: key_down,
			_key_up: key_up,
			inner,
		}
    }
}

pub fn gamepad_to_controller_state(gp: &Gamepad) -> ControllerState {
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
