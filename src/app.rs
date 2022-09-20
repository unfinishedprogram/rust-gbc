use crate::{
	components::{
		log_view::log_view,
		memory_view::{memory_view, MemoryViewState},
		screen_view::{screen_view, ScreenViewState},
		state_view::state_view,
	},
	cpu::{registers::CPURegister16, Cpu},
	emulator::Emulator,
	memory::Memory,
	util::debug_draw::debug_draw_tile_data,
};
use poll_promise::Promise;

enum RomLoadType {
	Bios(Promise<ehttp::Result<Vec<u8>>>),
	Rom(Promise<ehttp::Result<Vec<u8>>>),
}

pub struct EmulatorManager {
	emulator: Emulator,

	loaded_file_data: Option<RomLoadType>,
	play: bool,
	logs: Vec<(u16, String)>,
	memory_view_state: MemoryViewState,
	screen_view_state: ScreenViewState,
	vram_view_state: ScreenViewState,
	page: usize,
}

impl Default for EmulatorManager {
	fn default() -> Self {
		Self {
			play: false,
			emulator: Emulator::new(),
			loaded_file_data: None::<RomLoadType>,
			logs: vec![],
			memory_view_state: MemoryViewState::default(),
			screen_view_state: ScreenViewState::default(),
			vram_view_state: ScreenViewState::new("VRAM"),
			page: 0,
		}
	}
}

impl EmulatorManager {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		Default::default()
	}

	pub fn step_cpu(&mut self) {
		let pc = self.emulator.cpu.registers.pc;
		let inst = self.emulator.cpu.execute_next_instruction();
		self.log(pc, format!("{:?}", inst));
	}

	pub fn log(&mut self, pc: u16, text: String) {
		if self.logs.len() >= 100 {
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
					self.emulator.cpu.load_boot_rom(rom.into_iter().as_slice());
					self.log(0, "Loaded BIOS".to_string());
					self.loaded_file_data = None;
				}
				_ => {}
			},
			Some(RomLoadType::Rom(rom)) => match rom.ready() {
				Some(Ok(rom)) => {
					self.emulator.cpu.load_cartridge(rom.into_iter().as_slice());
					self.log(0, "Loaded ROM".to_string());
					self.loaded_file_data = None;
				}
				_ => {}
			},
			_ => {}
		}

		state_view(ctx, &self.emulator.cpu);
		memory_view(ctx, &self.emulator.cpu, &mut self.memory_view_state);
		log_view(ctx, &self.logs);
		screen_view(ctx, &mut self.screen_view_state);
		screen_view(ctx, &mut self.vram_view_state);

		debug_draw_tile_data(
			&self.emulator.memory,
			&mut self.vram_view_state.pixel_buffer,
			self.page,
		);

		egui::SidePanel::left("side_panel").show(ctx, |ui| {
			ui.monospace(format!(
				"{}",
				self.emulator.memory.borrow().t_state.borrow()
			));

			if ui.button("Page Up").clicked() {
				self.page += 1;
			}
			if ui.button("Page Down").clicked() {
				self.page -= 1;
			}
			ui.monospace(format!("{}", self.page));

			if ui.button("step").clicked() {
				self.step_cpu();
			}

			if ui.button("Load Bios").clicked() {
				self.loaded_file_data.get_or_insert_with(|| {
					let ctx = ctx.clone();
					let (sender, promise) = Promise::new();
					// let request = ehttp::Request::get("dmg_boot.bin");
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
					let request = ehttp::Request::get("06-ld r,r.gb");

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
				// 70224 // t-cycles per frame
				while true {
					self.step_cpu();

					if self.emulator.cpu.registers.get_u16(CPURegister16::HL) == 0x8000 {
						self.play = false;
						break;
					}

					if self.emulator.cpu.registers.pc == 0x00E0 {
						self.log(0, "LOGO CHECK ROUTINE".to_string());
						self.play = false;
						break;
					}

					let mut mem_ref = self.emulator.memory.borrow();
					let mut t_state_ref = mem_ref.t_state.borrow_mut();

					if t_state_ref.to_owned() >= 702 {
						*t_state_ref = 0;
						break;
					}
				}
				ctx.request_repaint(); // wake up UI thread
			}
		});
	}
}

fn parse_response(response: ehttp::Response) -> Result<Vec<u8>, String> {
	Ok(response.bytes)
}
