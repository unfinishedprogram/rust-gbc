use std::{cell::RefCell, collections::HashSet, rc::Rc};

use gloo::{
	events::EventListener,
	utils::{document_element, window},
};
use wasm_bindgen::JsCast;
use web_sys::Gamepad;

use crate::emulator::controller::ControllerState;

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
			state += ControllerState::from_gamepad(&gp)
		}

		state
	}

	pub fn new() -> Self {
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
