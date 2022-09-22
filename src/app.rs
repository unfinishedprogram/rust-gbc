use crate::{
	cartridge::{CartridgeData, CartridgeType},
	components::{
		log_view::log_view,
		memory_view::{memory_view, MemoryViewState},
		screen_view::{screen_view, ScreenViewState},
		state_view::state_view,
	},
	cpu::registers::CPURegister16,
	emulator::Emulator,
	util::debug_draw::debug_draw_tile_data,
};

use poll_promise::Promise;

pub struct EmulatorManager {
	emulator: Emulator,
	loaded_file_data: Option<Promise<CartridgeData>>,
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
			loaded_file_data: None::<Promise<CartridgeData>>,
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
		if let Some(data) = &self.loaded_file_data {
			if let Some(result) = data.ready() {
				self.emulator.cpu.load_cartridge(result);
				self.log(0, "Loaded ROM".to_string());
				self.loaded_file_data = None;
			}
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
				self.load_cartridge_by_url("dmg_boot.bin", CartridgeType::BIOS);
			}

			if ui.button("Load Rom").clicked() {
				self.load_cartridge_by_url("06-ld r,r.gb", CartridgeType::ROM);
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
				loop {
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

					let mem_ref = self.emulator.memory.borrow();
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
