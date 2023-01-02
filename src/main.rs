use std::sync::{Arc, Mutex};

use gbc_emu::emulator::{lcd::LCD, EmulatorState};
use gloo_timers::callback::Interval;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{window, HtmlCanvasElement, ImageData};

fn main() {
	let emulator = {
		let mut state = EmulatorState::default();
		let lcd = LCD::default();
		state.bind_lcd(lcd);
		state
	};

	let emulator = Arc::new(Mutex::new(emulator));

	Interval::new(15, move || {
		let canvas: HtmlCanvasElement = window()
			.unwrap()
			.document()
			.unwrap()
			.query_selector("#screen")
			.unwrap()
			.expect("Didn't find the map canvas.")
			.dyn_into::<web_sys::HtmlCanvasElement>()
			.unwrap(); // cannot be other than a canvas

		let context = canvas
			.get_context("2d")
			.unwrap()
			.unwrap()
			.dyn_into::<web_sys::CanvasRenderingContext2d>()
			.unwrap();

		if let Ok(mut state) = emulator.lock() {
			let start = state.get_cycle();
			while state.get_cycle() - start < (0.015 * 1_048_576.0) as u64 {
				state.step();
			}

			let img_data = ImageData::new_with_u8_clamped_array(
				Clamped(state.lcd.as_ref().unwrap().get_current_as_bytes()),
				160,
			)
			.unwrap();

			context.put_image_data(&img_data, 0.0, 0.0).unwrap();
		}
	})
	.forget();
}
