mod input;
mod screen;
mod setup_listeners;
mod uploader;
mod web_save_manager;
pub use setup_listeners::setup_listeners;
mod util;
use gloo::{file::callbacks::FileReader, net::http::Request, timers::callback::Interval};
use screen::get_screen_ctx;

use std::{cell::RefCell, fmt::Display};

use wasm_bindgen::Clamped;
use web_sys::ImageData;

use gameboy::{
	lcd::LCD,
	save_state::{RomSource, SaveState},
	Gameboy,
};

use self::{input::InputState, uploader::setup_upload_listeners};

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
	// Keeps file reader state, should never be used internal only
	_file_reader: Option<FileReader>,
	emulator_state: Gameboy,
	running_state: RunningState,
	input_state: InputState,
}

impl Default for Application {
	fn default() -> Self {
		setup_upload_listeners();
		let emulator_state = {
			let mut state = Gameboy::default();
			let lcd = LCD::default();
			state.bind_lcd(lcd);
			state
		};

		Self {
			_file_reader: None,
			input_state: InputState::new(),
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
	}

	pub fn step_emulator(&mut self, delta: f64) {
		let start = self.emulator_state.get_cycle();

		while self.emulator_state.get_cycle() - start < (1.5 * delta * 1_048_576.0) as u64 {
			self.emulator_state.step();
		}
	}

	pub fn step_frame(&mut self) {
		let controller_state = self.input_state.get_controller_state();
		self.emulator_state.set_controller_state(&controller_state);
		self.step_emulator(0.015);
		self.render_screen()
	}

	pub fn start(&mut self) {
		let interval = Interval::new(15, || {
			APPLICATION.with_borrow_mut(|app| app.step_frame());
		});
		self.running_state = RunningState::Playing(interval);
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

	pub async fn load_rom_from_source(source: Option<RomSource>) -> Option<Vec<u8>> {
		let Some(RomSource::ExternalUrl(path) | RomSource::LocalUrl(path)) = source else {
			return None;
		};

		let resp = Request::get(&path).send().await.unwrap();
		let rom_data: Vec<_> = resp.binary().await.unwrap();
		Some(rom_data)
	}

	pub fn load_rom(&mut self, rom: &[u8], source: Option<RomSource>) {
		self.emulator_state = Gameboy::default();
		let lcd = LCD::new();
		self.emulator_state.bind_lcd(lcd);
		self.emulator_state.load_rom(rom, source).unwrap();
	}

	pub fn load_save_state_with_rom(&mut self, rom: &[u8], save: SaveState) {
		self.load_rom(rom, save.rom_source.clone());
		self.emulator_state = self.emulator_state.clone().load_save_state(save);
		self.emulator_state.bind_lcd(LCD::new());
	}

	pub async fn load_save_state(&mut self, save: SaveState) {
		let rom = Self::load_rom_from_source(save.rom_source.clone()).await;
		self.load_save_state_with_rom(&rom.unwrap(), save);
	}
}
