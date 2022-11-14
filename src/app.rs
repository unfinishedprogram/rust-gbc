pub mod components;
pub mod drawable;
pub mod managed_input;
mod style;

use components::{draw_status, logger, Debugger};
use poll_promise::Promise;

pub struct EmulatorManager {
	loaded_file_data: Option<Promise<Vec<u8>>>,
	roms: Vec<&'static str>,
	debugger: Debugger,
}

impl Default for EmulatorManager {
	fn default() -> Self {
		Self {
			loaded_file_data: None::<Promise<Vec<u8>>>,
			debugger: Debugger::default(),
			roms: vec![
				"roms/tetris.gb",
				"roms/dr-mario.gb",
				"roms/02-interrupts.gb",
				"roms/06-ld r,r.gb",
				"roms/07-jr,jp,call,ret,rst.gb",
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
				match result {
					Ok(data) => sender.send(data),
					_ => {}
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
				self.debugger.emulator_state.load_rom(rom);
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
			unsafe { logger::draw(ui) };
		});

		egui::SidePanel::right("right_panel").show(ctx, |ui| self.debugger.draw(ui));

		egui::CentralPanel::default().show(ctx, |ui| ui.heading("Central Panel"));

		ctx.request_repaint()
	}
}

fn parse_response(response: ehttp::Response) -> Result<Vec<u8>, String> {
	Ok(response.bytes)
}
