use crate::{
	components::{
		log_view::log_view,
		memory_view::{memory_view, MemoryViewState},
		state_view::state_view,
	},
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
	logs: Vec<(u16, String)>,
	memory_view_state: MemoryViewState,
}

impl Default for EmulatorManager {
	fn default() -> Self {
		Self {
			play: false,
			cpu: Cpu::new(),
			loaded_file_data: None::<RomLoadType>,
			logs: vec![],
			memory_view_state: MemoryViewState::default(),
		}
	}
}

impl EmulatorManager {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		Default::default()
	}

	pub fn step_cpu(&mut self) {
		let inst = self.cpu.execute_next_instruction();
		self.log(format!("{:?}", inst));
	}

	pub fn log(&mut self, text: String) {
		let pc = self.cpu.registers.pc;
		if (self.logs.len() >= 100) {
			self.logs.remove(0);
		}
		self.logs.push((pc, text));
	}
}

impl eframe::App for EmulatorManager {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		match &self.loaded_file_data {
			Some(RomLoadType::Bios(rom)) => match rom.ready() {
				Some(Ok(rom)) => {
					self.cpu.load_boot_rom(rom.into_iter().as_slice());
					self.log("Loaded BIOS".to_string());
					self.loaded_file_data = None;
				}
				_ => {}
			},
			Some(RomLoadType::Rom(rom)) => match rom.ready() {
				Some(Ok(rom)) => {
					self.cpu.load_cartridge(rom.into_iter().as_slice());
					self.log("Loaded ROM".to_string());
					self.loaded_file_data = None;
				}
				_ => {}
			},
			_ => {}
		}

		state_view(ctx, &self.cpu);
		memory_view(ctx, &self.cpu, &mut self.memory_view_state);
		log_view(ctx, &self.logs);

		egui::SidePanel::left("side_panel").show(ctx, |ui| {
			if ui.button("step").clicked() {
				self.step_cpu();
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

			if ui
				.button(match self.play {
					true => "stop",
					false => "start",
				})
				.clicked()
			{
				self.play = !self.play
			}

			if self.play {
				for i in 0..4 {
					self.step_cpu();
				}
				ctx.request_repaint(); // wake up UI thread
			}
		});
	}
}

fn parse_response(response: ehttp::Response) -> Result<Vec<u8>, String> {
	Ok(response.bytes)
}
