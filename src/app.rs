use crate::{
	cartridge::{CartridgeData, CartridgeType},
	components::{
		buffer_view::{render_image, BufferViewState},
		joypad_view::joypad_view,
		log_view::log_view,
		memory_view::{memory_view, MemoryViewState},
		status_view::status_view,
	},
	cpu::{instruction::Instruction, registers::CPURegister16},
	emulator::Emulator,
	util::debug_draw::{debug_draw_tile_data, debug_draw_window_data},
};

use eframe::epaint::Shadow;
use egui::{style::Widgets, Rounding, Stroke, Style};
use egui::{Color32, Visuals};
use poll_promise::Promise;

pub struct EmulatorManager {
	emulator: Emulator,
	loaded_file_data: Option<Promise<CartridgeData>>,
	play: bool,
	logs: Vec<(u16, String)>,
	memory_view_state: MemoryViewState,
	tile_view_state: BufferViewState,
	vram_view_state: BufferViewState,
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
			tile_view_state: BufferViewState::new("Window View", (256, 256)),
			vram_view_state: BufferViewState::new("VRAM View", (256, 256)),
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

	pub fn step_emulation(&mut self) {
		let pc = self.emulator.cpu.registers.pc;
		if let Some(inst) = self.emulator.step() {
			self.log(pc, format!("{:?}", inst));
		};
	}

	pub fn log(&mut self, pc: u16, text: String) {
		if self.logs.len() >= 1000 {
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

fn color(val: u32) -> Color32 {
	let [_, r, g, b] = val.to_be_bytes();
	Color32::from_rgb(r as u8, g as u8, b as u8)
}

impl eframe::App for EmulatorManager {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let mut visuals = Visuals::default();
		let mut widgets = Widgets::default();
		let mut style = Style::default();

		style.spacing.item_spacing = (5.0, 5.0).into();
		style.spacing.button_padding = (10.0, 5.0).into();

		widgets.noninteractive.bg_fill = color(0x1c212b);
		widgets.noninteractive.bg_stroke = Stroke::new(1.0, color(0xBBBBBB));

		widgets.inactive.rounding = Rounding::default().at_least(2.0);

		visuals.widgets = widgets;
		visuals.window_rounding = Rounding::default().at_least(2.0);
		visuals.window_shadow = Shadow::small_dark();
		visuals.override_text_color = Some(color(0xc5c5c5));
		visuals.hyperlink_color = color(0x0096cf);

		ctx.set_style(style);
		ctx.set_visuals(visuals);

		if let Some(data) = &self.loaded_file_data {
			if let Some(result) = data.ready() {
				self.emulator.cpu.load_cartridge(result);
				self.log(0, "Loaded ROM".to_string());
				self.loaded_file_data = None;
			}
		}

		joypad_view(ctx, &mut self.emulator.cpu);
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

		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.menu_button("file", |ui| {
					ui.menu_button("Load Rom", |ui| {
						for rom in self.roms.clone().iter() {
							if ui.button(rom.to_string()).clicked() {
								ui.add_space(5.0);
								self.load_cartridge_by_url(rom, CartridgeType::ROM);
								ui.close_menu();
							}
						}
					});
				});

				if ui.button("step").clicked() || ctx.input().key_pressed(egui::Key::ArrowRight) {
					self.step_emulation();
				}

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
			log_view(ui, &self.logs);
		});

		egui::SidePanel::right("right_panel").show(ctx, |ui| {
			ui.horizontal(|ui| {
				status_view(ui, &self.emulator);
				memory_view(ui, &self.emulator.cpu, &mut self.memory_view_state);
			})
		});

		egui::CentralPanel::default().show(ctx, |ui| ui.heading("Central Panel"));

		if self.play {
			// 70224 // t-cycles per frame
			let mut count = 0;
			loop {
				self.step_emulation();
				count += 1;
				// if self.emulator.cpu.registers.get_u16(CPURegister16::PC) == 0x37 {
				// self.play = false;
				// break;
				// }

				if count > 7022 {
					break;
				}
			}
			ctx.request_repaint(); // wake up UI thread
		}
	}
}

fn parse_response(response: ehttp::Response) -> Result<Vec<u8>, String> {
	Ok(response.bytes)
}
