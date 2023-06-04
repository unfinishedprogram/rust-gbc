
use gloo::{
	file::callbacks::FileReader, net::http::Request, timers::callback::Interval, utils::document,
};


use std::{cell::RefCell, fmt::Display};

use wasm_bindgen::Clamped;
use web_sys::ImageData;

use gameboy::{
	save_state::{RomSource, SaveState},
	Gameboy, Mode,
};

use crate::{screen, input::InputState};
use screen::get_screen_ctx;

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
	pub _file_reader: Option<FileReader>,
	pub emulator_state: Gameboy,
	pub running_state: RunningState,
	input_state: InputState,
	frame_counts: Vec<u64>,
	frame_times: Vec<f64>,
	speed_multiplier: f64,
}

impl Default for Application {
	fn default() -> Self {
		let emulator_state = Gameboy::default();

		Self {
			frame_counts: vec![0; 30],
			frame_times: vec![0.0; 30],
			_file_reader: None,
			input_state: InputState::default(),
			running_state: RunningState::Paused,
			emulator_state,
			speed_multiplier: 1.0,
		}
	}
}

impl Application {
	pub fn render_screen(&mut self) {
		let screen = &self.emulator_state.ppu.lcd;

		let img_data =
			ImageData::new_with_u8_clamped_array(Clamped(screen.front_buffer()), 160).unwrap();

		get_screen_ctx()
			.put_image_data(&img_data, 0.0, 0.0)
			.unwrap();
	}

	pub fn step_fast(&mut self, delta: f64) {
		let perf = gloo::utils::window().performance().unwrap();
		let start_time = perf.now();

		let start_frame = self.emulator_state.ppu.frame;

		while perf.now() - start_time < delta {
			for _ in 0..1024 {
				self.emulator_state.step();
			}
		}

		let end_frame = self.emulator_state.ppu.frame;
		let frames = end_frame - start_frame;

		self.frame_counts.remove(0);
		self.frame_times.remove(0);

		self.frame_counts.push(frames);
		self.frame_times.push(perf.now() - start_time);

		let frames: u64 = self.frame_counts.iter().sum();
		let time: f64 = self.frame_times.iter().sum();

		let _state_text = if let Mode::GBC(state) = &self.emulator_state.mode {
			format!("{:?}", state.current_speed())
		} else {
			"".to_owned()
		};

		gloo::console::log!(frames as f64 / (time / 1000.0));
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
		self.step_emulator(0.015 * self.speed_multiplier);
		// self.step_fast(15.0);
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

	pub fn step_single(&mut self) {
		document()
			.query_selector("#debug_info")
			.unwrap()
			.unwrap()
			.set_inner_html(&format!("{:?}", self.emulator_state.cpu_state.registers));

		self.emulator_state.step();
		self.render_screen();
	}

	pub fn run_until_boot(&mut self) {
		self.emulator_state.run_until_boot();
		self.render_screen();
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
		self.emulator_state.load_rom(rom, source);
	}

	pub fn load_save_state_with_rom(&mut self, rom: &[u8], save: SaveState) {
		self.load_rom(rom, save.rom_source.clone());
		self.emulator_state = self.emulator_state.clone().load_save_state(save);
	}

	pub async fn load_save_state(&mut self, save: SaveState) {
		let rom = Self::load_rom_from_source(save.rom_source.clone()).await;
		self.load_save_state_with_rom(&rom.unwrap(), save);
	}

	pub fn set_speed(&mut self, multiplier: f64) {
		self.speed_multiplier = multiplier;
	}
}
