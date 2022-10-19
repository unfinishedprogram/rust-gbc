pub mod components;
pub mod drawable;
pub mod managed_input;
use crate::emulator::{
	cartridge::{CartridgeData, CartridgeType},
	Emulator,
};
use crate::util::{
	color::color,
	debug_draw::{debug_draw_tile_data, debug_draw_window_data},
};
use components::{joypad_view::joypad_view, BufferView, Debugger};
use drawable::*;

use eframe::epaint::Shadow;
use egui::Visuals;
use egui::{style::Widgets, Rounding, Stroke, Style};
use poll_promise::Promise;

use self::components::logger;

pub struct EmulatorManager {
	emulator: Emulator,
	loaded_file_data: Option<Promise<CartridgeData>>,
	play: bool,
	tile_view_state: BufferView,
	vram_view_state: BufferView,
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
			tile_view_state: BufferView::new("Window View", (256, 256)),
			vram_view_state: BufferView::new("VRAM View", (256, 256)),
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
				logger::info("Loaded ROM");
				self.loaded_file_data = None;
			}
		}

		joypad_view(ctx, &mut self.emulator.cpu);

		// render_image(ctx, &mut self.vram_view_state);
		// render_image(ctx, &mut self.tile_view_state);

		egui::Window::new("vram")
			.resizable(false)
			.show(ctx, |ui| self.vram_view_state.draw(ui));

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

		unsafe {
			egui::SidePanel::left("left_panel").show(ctx, |ui| logger::draw(ui));
		}

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
