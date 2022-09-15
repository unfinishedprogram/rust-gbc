use crate::{
	components::{memory_view::memory_view, state_view::state_view},
	cpu::Cpu,
};
use poll_promise::Promise;

enum RomLoadType {
	Bios(Promise<ehttp::Result<Vec<u8>>>),
	Rom(Promise<ehttp::Result<Vec<u8>>>),
}

pub struct EmulatorManager {
	cpu: Cpu,
	loaded_file_data: Option<RomLoadType>,
	play: bool,
}

impl Default for EmulatorManager {
	fn default() -> Self {
		Self {
			play: false,
			cpu: Cpu::new(),
			loaded_file_data: None::<RomLoadType>,
		}
	}
}

impl EmulatorManager {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		Default::default()
	}
}

impl eframe::App for EmulatorManager {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let Self {
			play,
			cpu,
			loaded_file_data,
		} = self;

		match &self.loaded_file_data {
			Some(RomLoadType::Bios(rom)) => match rom.ready() {
				Some(rom) => {
					self.cpu
						.load_boot_rom(rom.as_ref().unwrap().into_iter().as_slice());
					self.loaded_file_data = None;
				}
				None => {}
			},
			Some(RomLoadType::Rom(rom)) => match rom.ready() {
				Some(rom) => {
					self.cpu
						.load_cartridge(rom.as_ref().unwrap().into_iter().as_slice());
					self.loaded_file_data = None;
				}
				None => {}
			},
			None => {}
		}

		state_view(ctx, &self.cpu);
		memory_view(ctx, &self.cpu);

		egui::SidePanel::left("side_panel").show(ctx, |ui| {
			if ui.button("step").clicked() {
				self.cpu.execute_next_instruction();
			}

			if ui.button("Load Bios").clicked() {
				self.loaded_file_data.get_or_insert_with(|| {
					let ctx = ctx.clone();
					let (sender, promise) = Promise::new();
					let request = ehttp::Request::get("dmg_boot.bin");
					ehttp::fetch(request, move |response| {
						let data = response.and_then(parse_response);
						sender.send(data); // send the results back to the UI thread.
						ctx.request_repaint(); // wake up UI thread
					});

					RomLoadType::Bios(promise)
				});
			}

			if ui.button("Load Rom").clicked() {
				self.loaded_file_data.get_or_insert_with(|| {
					let ctx = ctx.clone();
					let (sender, promise) = Promise::new();
					let request = ehttp::Request::get("tetris.gb");
					ehttp::fetch(request, move |response| {
						let data = response.and_then(parse_response);
						sender.send(data); // send the results back to the UI thread.
						ctx.request_repaint(); // wake up UI thread
					});

					RomLoadType::Rom(promise)
				});
			}

			match self.play {
				false => {
					if ui.button("start").clicked() {
						self.play = true;
					}
				}
				true => {
					if ui.button("stop").clicked() {
						self.play = false
					}
				}
			}

			if self.play {
				self.cpu.execute_next_instruction();
				ctx.request_repaint(); // wake up UI thread
			}
		});
	}
}

fn parse_response(response: ehttp::Response) -> Result<Vec<u8>, String> {
	Ok(response.bytes)
}
