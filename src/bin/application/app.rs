use gloo::{file::callbacks::FileReader, net::http::Request};
use std::{cell::RefCell, collections::VecDeque, fmt::Display};

use wasm_bindgen::{prelude::wasm_bindgen, Clamped};
use web_sys::ImageData;

use gameboy::{
	save_state::{RomSource, SaveState},
	Gameboy,
};

use crate::{
	// audio::{self, AudioHandler},
	input::InputState,
};

thread_local! {
	pub static APPLICATION: RefCell<Application> = RefCell::new(Application::default());
}

pub enum RunningState {
	Playing,
	Paused,
}

fn performance_now() -> f64 {
	gloo::utils::window().performance().unwrap().now()
}

impl Display for RunningState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			RunningState::Playing => write!(f, "Play"),
			RunningState::Paused => write!(f, "Pause"),
		}
	}
}

#[wasm_bindgen]
pub struct Application {
	// Keeps file reader state, should never be used internal only
	_file_reader: Option<FileReader>,
	pub(crate) emulator_state: Gameboy,
	running_state: RunningState,
	pub previous_frame_time: f64,
	// pub(crate) audio: Option<AudioHandler>,
	input_state: InputState,
	speed_multiplier: f64,
	frames: VecDeque<f64>,
}

impl Default for Application {
	fn default() -> Self {
		let emulator_state = Gameboy::default();
		Self {
			previous_frame_time: 0.0,
			_file_reader: None,
			input_state: InputState::default(),
			running_state: RunningState::Paused,
			emulator_state,
			speed_multiplier: 1.0,
			frames: VecDeque::with_capacity(30),
		}
	}
}

#[wasm_bindgen]
impl Application {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		Application::default()
	}

	#[wasm_bindgen]
	pub fn render_screen(&mut self) -> ImageData {
		let screen = &self.emulator_state.ppu.lcd;
		ImageData::new_with_u8_clamped_array(Clamped(screen.front_buffer()), 160).unwrap()
	}

	// Should be synched using request_animation_frame
	// Better responsiveness / no-frame tearing
	#[wasm_bindgen]
	pub fn step_lcd_frame(&mut self, elapsed: f64) -> f64 {
		// We clamp to avoid issues when tabbing out and back in to the tab
		let delta_t = (elapsed - self.previous_frame_time).min(32.0);
		self.previous_frame_time = elapsed;

		let controller_state = self.input_state.get_controller_state();
		self.emulator_state.set_controller_state(&controller_state);

		let iters = if self.speed_multiplier > 1.0 {
			self.speed_multiplier.round() as i32
		} else {
			1
		};

		for _ in 0..iters {
			let mut steps = 0;
			let start_frame = self.emulator_state.ppu.frame;
			while steps < 10_000_000 {
				steps += 1;
				self.emulator_state.step();
				if self.emulator_state.ppu.frame != start_frame {
					self.frames.push_back(performance_now());
					break;
				}
			}
		}

		delta_t

		// if let Some(audio) = &mut self.audio {
		// 	audio.pull_samples(&mut self.emulator_state.audio, delta_t);
		// } else if self.input_state.get_controller_state().as_byte() != 255 {
		// 	log::error!(
		// 		"Audio not initialized {:?}",
		// 		self.input_state.get_controller_state().as_byte()
		// 	);
		// 	match audio::AudioHandler::new() {
		// 		Ok(mut audio) => {
		// 			audio.play();
		// 			self.audio = Some(audio);
		// 		}
		// 		Err(err) => {
		// 			log::error!("Failed to create audio context: {:?}", err);
		// 		}
		// 	}
		// }
	}

	#[wasm_bindgen]
	pub fn start(&mut self) {
		self.running_state = RunningState::Playing;
	}

	#[wasm_bindgen]
	pub fn stop(&mut self) {
		self.running_state = RunningState::Paused;
	}

	#[wasm_bindgen]
	pub fn pull_audio_samples(&mut self, samples: usize) -> Vec<f32> {
		let samples = self.emulator_state.audio.pull_samples(samples);

		// TODO: Fix copying
		samples.iter().flat_map(|(l, r)| vec![*l, *r]).collect()
	}

	pub fn toggle_play(&mut self) -> bool {
		use RunningState::*;

		match self.running_state {
			Playing => self.stop(),
			Paused => self.start(),
		};

		match self.running_state {
			Playing => true,
			Paused => false,
		}
	}

	pub fn step_single(&mut self) {
		self.emulator_state.step();
	}

	pub fn run_until_boot(&mut self) {
		self.emulator_state.run_until_boot();
	}

	pub async fn load_rom_from_source(source: Option<String>) -> Option<Vec<u8>> {
		let path = source?;

		let resp = Request::get(&path).send().await.unwrap();
		let rom_data: Vec<_> = resp.binary().await.unwrap();
		Some(rom_data)
	}

	pub fn load_rom(&mut self, rom: &[u8], source: Option<String>) {
		self.emulator_state = Gameboy::default();

		self.emulator_state
			.load_rom(rom, source.map(RomSource::LocalUrl));
	}

	pub(crate) fn load_save_state_with_rom(&mut self, rom: &[u8], save: SaveState) {
		let path = save.rom_source.clone().map(|source| match source {
			RomSource::LocalUrl(path) => path,
			RomSource::ExternalUrl(path) => path,
		});

		self.load_rom(rom, path);
		self.emulator_state = self.emulator_state.clone().load_save_state(save);
	}

	pub fn set_speed(&mut self, multiplier: f64) {
		self.speed_multiplier = multiplier;
	}
}
