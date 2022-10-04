pub mod components;
pub mod drawable;
pub mod managed_input;

use components::{
	buffer_view::{render_image, BufferViewState},
	joypad_view::joypad_view,
	logger::Logger,
	memory_view::MemoryView,
	status_view::status_view,
};
use drawable::DrawableMut;

use crate::{
	cartridge::{CartridgeData, CartridgeType},
	emulator::Emulator,
	util::{
		color::color,
		debug_draw::{debug_draw_tile_data, debug_draw_window_data},
	},
};

use eframe::epaint::Shadow;
use egui::Visuals;
use egui::{style::Widgets, Rounding, Stroke, Style};
use poll_promise::Promise;

use self::components::breakpoint_manager::{self, BreakpointManager};

pub struct EmulatorManager {
	emulator: Emulator,
	logger: Logger,
	loaded_file_data: Option<Promise<CartridgeData>>,
	play: bool,
	memory_view: MemoryView,
	tile_view_state: BufferViewState,
	vram_view_state: BufferViewState,
	breakpoint_manager: BreakpointManager,
	roms: Vec<&'static str>,
}

impl Default for EmulatorManager {
	fn default() -> Self {
		let emulator = Emulator::new();
		let mut breakpoint_manager = BreakpointManager::default();

		Self {
			play: false,
			memory_view: MemoryView::new(emulator.memory.clone(), &mut breakpoint_manager),
			breakpoint_manager,
			loaded_file_data: None::<Promise<CartridgeData>>,
			logger: Logger::default(),
			emulator,
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
			self.logger.info(format!("{} : {:?}", pc, inst));
			self.memory_view.focus_cell(pc as usize);
		};
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
		ctx.set_debug_on_hover(true);

		if let Some(data) = &self.loaded_file_data {
			if let Some(result) = data.ready() {
				self.emulator.cpu.load_cartridge(result);
				self.logger.info("Loaded ROM");
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

		egui::SidePanel::left("left_panel").show(ctx, |ui| self.logger.draw(ui));

		egui::SidePanel::right("right_panel").show(ctx, |ui| {
			ui.vertical(|ui| {
				self.breakpoint_manager.draw(ui);
				ui.horizontal_top(|ui| {
					status_view(ui, &self.emulator);
					self.memory_view.draw(ui);
				})
			})
		});

		egui::CentralPanel::default().show(ctx, |ui| ui.heading("Central Panel"));

		if self.play {
			// 70224 // t-cycles per frame
			let mut count = 0;
			loop {
				self.step_emulation();
				if self
					.breakpoint_manager
					.break_on(self.emulator.cpu.registers.pc)
				{
					self.play = false;
					self.logger.debug("Breaking");
				}
				count += 1;
				if count > 0 {
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
