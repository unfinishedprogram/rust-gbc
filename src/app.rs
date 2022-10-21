pub mod components;
pub mod drawable;
pub mod managed_input;
mod style;

use crate::emulator::{
	cartridge::{CartridgeData, CartridgeType},
	Emulator,
};

use components::{draw_cpu_status, joypad_view::joypad_view, logger, Debugger};
use poll_promise::Promise;

pub struct EmulatorManager {
	emulator: Emulator,
	loaded_file_data: Option<Promise<CartridgeData>>,
	play: bool,
	roms: Vec<&'static str>,
	debugger: Debugger,
}

impl Default for EmulatorManager {
	fn default() -> Self {
		let emulator = Emulator::new();

		Self {
			play: false,
			loaded_file_data: None::<Promise<CartridgeData>>,
			debugger: Debugger::default(),
			roms: vec![
				"roms/tetris.gb",
				"roms/dr-mario.gb",
				"roms/02-interrupts.gb",
				"roms/06-ld r,r.gb",
				"roms/07-jr,jp,call,ret,rst.gb",
			],
			emulator,
		}
	}
}

impl EmulatorManager {
	pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		Default::default()
	}

	pub fn load_cartridge_by_url(&mut self, url: &str, cartridge_type: CartridgeType) {
		self.loaded_file_data.get_or_insert_with(|| {
			let (sender, promise) = Promise::new();

			let request = ehttp::Request::get(url);

			ehttp::fetch(request, move |response| {
				let result = response.and_then(parse_response);
				match result {
					Ok(data) => sender.send((cartridge_type, data)),
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
			if let Some(result) = data.ready() {
				self.emulator.cpu.load_cartridge(result);
				logger::info("Loaded ROM");
				self.loaded_file_data = None;
			}
		}

		joypad_view(ctx, &mut self.emulator.cpu);

		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.menu_button("file", |ui| {
					ui.menu_button("load rom", |ui| {
						for rom in &self.roms.clone() {
							if ui.button(rom.to_string()).clicked() {
								ui.add_space(5.0);
								self.load_cartridge_by_url(rom, CartridgeType::ROM);
								ui.close_menu();
							}
						}
					});
				});

				if ui.button("Load Bios").clicked() {
					self.load_cartridge_by_url("roms/dmg_boot.bin", CartridgeType::BIOS);
				}

				if ui
					.button(match self.play {
						true => "stop",
						false => "start",
					})
					.clicked()
				{
					self.play = !self.play
				}
			})
		});

		egui::SidePanel::left("left_panel").show(ctx, |ui| {
			ui.vertical(|ui| draw_cpu_status(ui, &self.emulator));
			unsafe { logger::draw(ui) };
		});

		egui::SidePanel::right("right_panel")
			.show(ctx, |ui| self.debugger.draw(&mut self.emulator, ui));

		egui::CentralPanel::default().show(ctx, |ui| ui.heading("Central Panel"));

		if self.play {
			self.debugger.step(702, &mut self.emulator);
			ctx.request_repaint(); // wake up UI thread
		}
	}
}

fn parse_response(response: ehttp::Response) -> Result<Vec<u8>, String> {
	Ok(response.bytes)
}
