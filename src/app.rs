pub mod components;
mod controller;
pub mod drawable;
mod file_selector;
pub mod logger;
pub mod managed_input;
mod save_manager;
mod style;
mod web_save_manager;
use crate::{
	app::file_selector::file_selector,
	emulator::{lcd::LCD, EmulatorState},
};

use std::sync::Mutex;
use wasm_bindgen::JsCast;
use web_sys::{window, Gamepad};

use components::{draw_status, Debugger};
use egui::{show_tooltip, CentralPanel, Id, SidePanel, TopBottomPanel};
use lazy_static::lazy_static;
use poll_promise::Promise;

use crate::util::file_types::Entry;

use self::{
	components::log_view::draw_logs,
	controller::ControllerState,
	drawable::DrawableMut,
	logger::Logger,
	save_manager::{SaveError, SaveManager, SaveState},
	web_save_manager::WebSaveManager,
};

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

	fn set_input_state(&mut self, state: ControllerState) {
		let _last_input = self.debugger.emulator_state.raw_joyp_input;
		self.debugger.emulator_state.raw_joyp_input = state.as_byte();
		// TODO
		// Add Interrupt handling
	}

	fn get_gamepad(&self) -> Option<Gamepad> {
		window()?
			.navigator()
			.get_gamepads()
			.ok()?
			.get(0)
			.dyn_into::<Gamepad>()
			.ok()
	}

	fn fetch_input_state(&self, ctx: &egui::Context) -> ControllerState {
		if let Some(gp) = &self.get_gamepad() {
			gp.into()
		} else {
			let keys = &ctx.input().keys_down;
			keys.into()
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

	pub fn load_save_state(&mut self, state: SaveState) {
		let mut state: EmulatorState = serde_json::from_str::<EmulatorState>(&state.data).unwrap();
		state.bind_lcd(LCD::new());
		self.debugger.emulator_state = state;
	}

	pub fn create_new_save_state(&self) -> Result<SaveState, SaveError> {
		SaveState::try_from(&self.debugger.emulator_state)
	}
}

impl eframe::App for EmulatorManager {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		ctx.request_repaint();
		style::apply(ctx);
		self.set_input_state(self.fetch_input_state(ctx));

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

				if ui.button("Save").clicked() {
					if let Ok(state) = self.create_new_save_state() {
						_ = WebSaveManager::save_save_state(state);
					} else {
						show_tooltip(ui.ctx(), Id::new("save_error"), |ui| {
							ui.label("Error Saving State")
						});
					}
				}

				ui.menu_button("Load", |ui| {
					if let Ok(states) = WebSaveManager::load_save_states() {
						for state in states {
							if ui.button(format!("{state}")).clicked() {
								self.load_save_state(state);
								ui.close_menu();
								self.debugger.start();
							}
						}
					} else {
						ui.label("Error Loading Saves");
					};
				});

				ui.checkbox(&mut self.debug, "Debug");
			})
		});

		self.debugger.step(ctx.input().unstable_dt.min(0.5));

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
	}
}

fn parse_response(response: ehttp::Response) -> Result<Vec<u8>, String> {
	Ok(response.bytes)
}
