pub mod components;
pub mod drawable;
mod file_selector;
pub mod logger;
pub mod managed_input;
mod style;
use crate::{
	app::file_selector::file_selector,
	emulator::{flags::INT_JOY_PAD, lcd::LCD},
};

use std::sync::Mutex;

use components::{draw_status, Debugger};
use egui::{CentralPanel, Key, SidePanel, TopBottomPanel};
use lazy_static::lazy_static;
use poll_promise::Promise;

use crate::util::{bits::bit, file_types::Entry};

use self::{components::log_view::draw_logs, drawable::DrawableMut, logger::Logger};

static LOGGER: Logger = Logger {
	logs: Mutex::new(vec![]),
};

lazy_static! {
	static ref ROMS: Entry = serde_json::from_str::<Entry>(include_str!("../roms.json")).unwrap();
}

pub struct EmulatorManager {
	loaded_file_data: Option<Promise<Vec<u8>>>,
	pub debugger: Debugger,
	logger: &'static Logger,
	debug: bool,
}

impl Default for EmulatorManager {
	fn default() -> Self {
		log::set_logger(&LOGGER).unwrap();
		log::set_max_level(log::LevelFilter::Debug);
		Self {
			debug: false,
			logger: &LOGGER,
			loaded_file_data: None::<Promise<Vec<u8>>>,
			debugger: Debugger::default(),
		}
	}
}

impl EmulatorManager {
	pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		let mut res = EmulatorManager::default();
		res.debugger.emulator_state.bind_lcd(LCD::new());
		res
	}

	fn update_key_input(&mut self, ctx: &egui::Context) {
		use Key::*;
		let keys = [
			Z, X, Space, Enter, ArrowRight, ArrowLeft, ArrowUp, ArrowDown,
		];

		let last_input = self.debugger.emulator_state.raw_joyp_input;

		self.debugger.emulator_state.raw_joyp_input = 0xFF;

		for (index, key) in keys.into_iter().enumerate() {
			if ctx.input().key_down(key) {
				self.debugger.emulator_state.raw_joyp_input &= !bit(index as u8);

				if last_input & bit(index as u8) != 0 {
					self.debugger.emulator_state.request_interrupt(INT_JOY_PAD);
				}
			};
		}
	}

	pub fn load_cartridge_by_url(&mut self, url: &str) {
		self.loaded_file_data.get_or_insert_with(|| {
			let (sender, promise) = Promise::new();

			let request = ehttp::Request::get(url);

			ehttp::fetch(request, move |response| {
				let result = response.and_then(parse_response);
				if let Ok(data) = result {
					sender.send(data)
				}
			});

			promise
		});
	}
}

impl eframe::App for EmulatorManager {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		style::apply(ctx);
		self.update_key_input(ctx);
		if let Some(data) = &self.loaded_file_data {
			if let Some(rom) = data.ready() {
				if let Err(e) = self.debugger.emulator_state.load_rom(rom) {
					log::error!("{e:?}");
				}
				self.loaded_file_data = None;
			}
		}

		TopBottomPanel::top("top_panel").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.menu_button("file", |ui| {
					file_selector(ui, &ROMS, &mut |selected| {
						self.load_cartridge_by_url(selected)
					});
				});

				if ui.button("Toggle Play").clicked() {
					self.debugger.toggle_state();
				}
				if ui.button("Step").clicked() {
					self.debugger.step_once();
				}

				ui.checkbox(&mut self.debug, "Debug");
			})
		});

		self.debugger.step();

		if self.debug {
			SidePanel::left("left_panel").show(ctx, |ui| {
				ui.vertical(|ui| draw_status(ui, &self.debugger.emulator_state));
				draw_logs(ui, &self.logger.logs.lock().unwrap());
			});

			SidePanel::right("right_panel").show(ctx, |ui| self.debugger.draw(ui));
		}

		CentralPanel::default().show(ctx, |ui| {
			if let Some(lcd) = self.debugger.emulator_state.lcd.as_mut() {
				lcd.draw_window(ui, "LCD");
			}
		});

		ctx.request_repaint()
	}
}

fn parse_response(response: ehttp::Response) -> Result<Vec<u8>, String> {
	Ok(response.bytes)
}
