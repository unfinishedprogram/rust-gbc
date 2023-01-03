mod screen;
mod setup_listeners;
pub use setup_listeners::setup_listeners;
mod util;
use gloo::timers::callback::Interval;
use screen::get_screen_ctx;

use std::{cell::RefCell, fmt::Display};

use wasm_bindgen::Clamped;
use web_sys::ImageData;

use crate::emulator::{lcd::LCD, EmulatorState};

thread_local! {
	pub static APPLICATION: RefCell<Application> = RefCell::new(Application::default());
}

pub enum RunningState {
	Playing(Interval),
	Paused,
}

impl Display for RunningState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			RunningState::Playing(_) => write!(f, "Play"),
			RunningState::Paused => write!(f, "Pause"),
		}
	}
}

pub struct Application {
	emulator_state: EmulatorState,
	running_state: RunningState,
}

impl Default for Application {
	fn default() -> Self {
		let emulator_state = {
			let mut state = EmulatorState::default();
			let lcd = LCD::default();
			state.bind_lcd(lcd);
			state
		};

		Self {
			running_state: RunningState::Paused,
			emulator_state,
		}
	}
}

impl Application {
	pub fn render_screen(&mut self) {
		let img_data = ImageData::new_with_u8_clamped_array(
			Clamped(
				self.emulator_state
					.lcd
					.as_ref()
					.unwrap()
					.get_current_as_bytes(),
			),
			160,
		)
		.unwrap();

		get_screen_ctx()
			.put_image_data(&img_data, 0.0, 0.0)
			.unwrap();
		gloo::console::log!("Rendering");
	}

	pub fn step_emulator(&mut self, delta: f64) {
		let start = self.emulator_state.get_cycle();

		while self.emulator_state.get_cycle() - start < (delta * 1_048_576.0) as u64 {
			self.emulator_state.step();
		}
	}

	pub fn start(&mut self) {
		self.running_state = RunningState::Playing(Interval::new(15, || {
			APPLICATION.with_borrow_mut(|app| {
				app.step_emulator(0.015);
				app.render_screen()
			});
		}));
	}

	pub fn stop(&mut self) {
		self.running_state = RunningState::Paused;
	}

	pub fn toggle_play(&mut self) {
		use RunningState::*;

		match self.running_state {
			Playing(_) => self.stop(),
			Paused => self.start(),
		};
	}
}
