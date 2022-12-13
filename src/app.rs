pub mod components;
pub mod drawable;
pub mod logger;
pub mod managed_input;
mod style;

use std::sync::Mutex;

use components::{draw_status, Debugger};
use poll_promise::Promise;

use self::{components::log_view::draw_logs, logger::Logger};

static LOGGER: Logger = Logger {
	logs: Mutex::new(vec![]),
};

pub struct EmulatorManager {
	loaded_file_data: Option<Promise<Vec<u8>>>,
	roms: Vec<&'static str>,
	debugger: Debugger,
	logger: &'static Logger,
}

impl Default for EmulatorManager {
	fn default() -> Self {
		log::set_logger(&LOGGER).unwrap();
		log::set_max_level(log::LevelFilter::Info);
		Self {
			logger: &LOGGER,
			loaded_file_data: None::<Promise<Vec<u8>>>,
			debugger: Debugger::default(),
			roms: vec![
				"roms/SuperMarioWorld.gb",
				"roms/instr_timing.gb",
				"roms/LegendOfZelda.gb",
				"roms/tetris.gb",
				"roms/tetris2.gb",
				"roms/dr-mario.gb",
				"roms/cpu_instrs.gb",
				"roms/PokemonRed.gb",
				"roms/01-special.gb",
				"roms/02-interrupts.gb",
				"roms/03-op sp,hl.gb",
				"roms/04-op r,imm.gb",
				"roms/05-op rp.gb",
				"roms/06-ld r,r.gb",
				"roms/07-jr,jp,call,ret,rst.gb",
				"roms/08-misc instrs.gb",
				"roms/09-op r,r.gb",
				"roms/10-bit ops.gb",
				"roms/11-op a,(hl).gb",
			],
		}
	}
}

impl EmulatorManager {
	pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		Default::default()
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
		if let Some(data) = &self.loaded_file_data {
			if let Some(rom) = data.ready() {
				if let Err(e) = self.debugger.emulator_state.load_rom(rom) {
					log::error!("{e:?}");
				}
				self.loaded_file_data = None;
			}
		}

		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.menu_button("file", |ui| {
					ui.menu_button("load rom", |ui| {
						for rom in &self.roms.clone() {
							if ui.button(rom.to_string()).clicked() {
								ui.add_space(5.0);
								self.load_cartridge_by_url(rom);
								ui.close_menu();
							}
						}
					});
				});

				if ui.button("Toggle Play").clicked() {
					self.debugger.toggle_state();
				}

				self.debugger.step();
			})
		});

		egui::SidePanel::left("left_panel").show(ctx, |ui| {
			ui.vertical(|ui| draw_status(ui, &self.debugger.emulator_state));
			draw_logs(ui, &self.logger.logs.lock().unwrap());
		});

		egui::SidePanel::right("right_panel").show(ctx, |ui| self.debugger.draw(ui));

		egui::CentralPanel::default().show(ctx, |ui| ui.heading("Central Panel"));

		ctx.request_repaint()
	}
}

fn parse_response(response: ehttp::Response) -> Result<Vec<u8>, String> {
	Ok(response.bytes)
}
