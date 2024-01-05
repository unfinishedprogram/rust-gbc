use crate::components::{
	run_controller::{self, RunController},
	show_system_info, CheckpointManager, JoypadInput, LinearMemoryView, MemoryImage, MemoryView,
	RomLoader, Screen, VramView,
};
use egui::{CentralPanel, SidePanel, Style, TextStyle, TopBottomPanel, Window};

use gameboy::Gameboy;

#[derive(Default)]
pub struct Debugger {
	gameboy: Gameboy,
	screen: Screen,
	run_controller: RunController,
	rom_loader: RomLoader,
	joypad: JoypadInput,

	// Debugging Windows
	checkpoint_manager: CheckpointManager,
	checkpoint_manager_enabled: bool,

	memory_image: MemoryImage,
	memory_image_enabled: bool,

	vram_view: VramView,
	vram_view_enabled: bool,

	memory_view: MemoryView,
	memory_view_enabled: bool,

	linear_memory_view: LinearMemoryView,
	linear_memory_view_enabled: bool,
}

impl Debugger {
	pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		Default::default()
	}
}

impl eframe::App for Debugger {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		self.gameboy.set_controller_state(self.joypad.update(ctx));

		ctx.set_style(Style {
			override_text_style: Some(TextStyle::Monospace),
			..Default::default()
		});

		TopBottomPanel::top("top").show(ctx, |ui| {
			ui.horizontal(|ui| {
				if let Some(action) = self.run_controller.draw(ui) {
					match action {
						run_controller::Action::StepFrame => {
							let start = self.gameboy.ppu.frame;
							while self.gameboy.ppu.frame == start {
								self.gameboy.step();
							}
						}
						run_controller::Action::Step(cycles) => {
							for _ in 0..cycles {
								self.gameboy.step();
							}
						}
						run_controller::Action::NextInterrupt => {
							while !(self.gameboy.cpu_state.interrupt_pending()
								&& (self.gameboy.cpu_state.ime() || self.gameboy.cpu_state.halted))
							{
								self.gameboy.step();
							}
						}
						run_controller::Action::SkipBios => {
							while self.gameboy.booting {
								self.gameboy.step();
							}
						}
						run_controller::Action::HDMAStart => {
							let mut steps = 0;
							while self.gameboy.dma_controller.read_hdma5() & 0b10000000 != 0
								&& steps < 10000000
							{
								steps += 1;
								self.gameboy.step();
							}
						}
						run_controller::Action::OAMDmaStart => todo!(),
					}
				}

				self.rom_loader.draw(ui, &mut self.gameboy);

				ui.checkbox(&mut self.checkpoint_manager_enabled, "Checkpoint Manager");
				ui.checkbox(&mut self.memory_image_enabled, "Memory Image");
				ui.checkbox(&mut self.vram_view_enabled, "Vram View");
				ui.checkbox(&mut self.memory_view_enabled, "Memory View");
				ui.checkbox(&mut self.linear_memory_view_enabled, "Instruction View");
			});
		});

		let screen_buffer = self.gameboy.ppu.lcd.front_buffer();

		SidePanel::right("right").show(ctx, |ui| show_system_info(&self.gameboy, ui));

		CentralPanel::default().show(ctx, |ui| self.screen.draw(ui, screen_buffer));

		if self.linear_memory_view_enabled {
			Window::new("Instructions")
				.show(ctx, |ui| self.linear_memory_view.draw(&self.gameboy, ui));
		}

		if self.memory_view_enabled {
			Window::new("Memory").show(ctx, |ui| self.memory_view.draw(&self.gameboy, ui));
		}
		if self.memory_image_enabled {
			Window::new("MemImage").show(ctx, |ui| self.memory_image.draw(&self.gameboy, ui));
		}

		if self.vram_view_enabled {
			Window::new("VramView").show(ctx, |ui| self.vram_view.draw(&self.gameboy, ui));
		}

		if self.checkpoint_manager_enabled {
			Window::new("Checkpoints").show(ctx, |ui| {
				self.checkpoint_manager.draw(&mut self.gameboy, ui)
			});
		}

		ctx.request_repaint();
	}
}
