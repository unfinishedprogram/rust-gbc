use crate::{
	cartridge::{CartridgeData, CartridgeType},
	components::{
		buffer_view::{render_image, BufferViewState},
		log_view::log_view,
		memory_view::{memory_view, MemoryViewState},
		state_view::state_view,
	},
	cpu::registers::CPURegister16,
	emulator::Emulator,
	util::debug_draw::{debug_draw_tile_data, debug_draw_window_data},
};

use egui::ComboBox;
use poll_promise::Promise;

pub struct EmulatorManager {
	emulator: Emulator,
	loaded_file_data: Option<Promise<CartridgeData>>,
	play: bool,
	logs: Vec<(u16, String)>,
	memory_view_state: MemoryViewState,
	screen_view_state: BufferViewState,
	tile_view_state: BufferViewState,
	vram_view_state: BufferViewState,
	selected_rom: &'static str,
	roms: Vec<&'static str>,
}

impl Default for EmulatorManager {
	fn default() -> Self {
		Self {
			play: false,
			emulator: Emulator::new(),
			loaded_file_data: None::<Promise<CartridgeData>>,
			logs: vec![],
			memory_view_state: MemoryViewState::default(),
			screen_view_state: BufferViewState::new("Screen View", (160, 144)),
			tile_view_state: BufferViewState::new("Window View", (256, 256)),
			vram_view_state: BufferViewState::new("VRAM View", (256, 256)),
			selected_rom: "",
			roms: vec!["roms/dr-mario.gb", "roms/06-ld r,r.gb", "roms/tetris.gb"],
		}
	}
}

impl EmulatorManager {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		Default::default()
	}

	pub fn step_emulation(&mut self) {
		let pc = self.emulator.cpu.registers.pc;

		if let Some(inst) = self.emulator.step() {
			self.log(pc, format!("{:?}", inst));
		};
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
		render_image(ctx, &mut self.screen_view_state);
		render_image(ctx, &mut self.vram_view_state);
		render_image(ctx, &mut self.tile_view_state);

		debug_draw_tile_data(
			&self.emulator.memory,
			&mut self.vram_view_state.pixel_buffer,
		);

		debug_draw_window_data(
			&self.emulator.memory,
			&mut self.tile_view_state.pixel_buffer,
		);

		egui::SidePanel::left("side_panel").show(ctx, |ui| {
			if ui.button("step").clicked() {
				self.step_emulation();
			}

			if ui.button("Load Bios").clicked() {
				self.load_cartridge_by_url("roms/dmg_boot.bin", CartridgeType::BIOS);
			}

			ui.horizontal(|ui| {
				ComboBox::from_label("")
					.selected_text(format!("{:?}", self.selected_rom))
					.show_ui(ui, |ui| {
						for rom in self.roms.iter() {
							ui.selectable_value(&mut self.selected_rom, rom, *rom);
						}
					});

				if ui.button("Load Rom").clicked() {
					self.load_cartridge_by_url(self.selected_rom, CartridgeType::ROM);
				}
			});

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
				let mut count = 0;
				loop {
					self.step_emulation();
					count += 1;
					if self.emulator.cpu.registers.get_u16(CPURegister16::PC) == 0xe9 {
						self.play = false;
						break;
					}

					if count > 7022 {
						count = 0;
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
