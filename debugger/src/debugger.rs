use crate::components::{
	run_controller::{self, RunController},
	show_cpu_info, show_ppu_info, LinearMemoryView, Logs, RomLoader, Screen,
};
use egui::{CentralPanel, SidePanel, Style, TextStyle, TopBottomPanel, Window};

use gameboy::Gameboy;

#[derive(Default)]
pub struct Debugger {
	gameboy: Gameboy,
	screen: Screen,
	// breakpoint_manager: BreakpointManager,
	run_controller: RunController,
	linear_memory_view: LinearMemoryView,
	rom_loader: RomLoader,
}

impl Debugger {
	pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		Default::default()
	}
}

impl eframe::App for Debugger {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		ctx.set_style(Style {
			override_text_style: Some(TextStyle::Monospace),
			..Default::default()
		});

		TopBottomPanel::top("top").show(ctx, |ui| {
			ui.horizontal(|ui| {
				use run_controller::Action;
				match self.run_controller.draw(ui) {
					Some(Action::StepFrame) => {
						for _ in 0..70224 {
							self.gameboy.step();
						}
					}
					Some(Action::StepSingle) => self.gameboy.step(),
					None => {}
				};
				self.rom_loader.draw(ui, &mut self.gameboy)
			});
		});

		let screen_buffer = self.gameboy.ppu.lcd.front_buffer();

		SidePanel::right("right").show(ctx, |ui| {
			Logs::draw(ui);
			// self.breakpoint_manager.draw(ui);
		});

		CentralPanel::default().show(ctx, |ui| self.screen.draw(ui, screen_buffer));

		Window::new("Memory").show(ctx, |ui| self.linear_memory_view.draw(&self.gameboy, ui));
		Window::new("CPU State").show(ctx, |ui| show_cpu_info(&self.gameboy, ui));
		Window::new("PPU State").show(ctx, |ui| show_ppu_info(&self.gameboy, ui));

		ctx.request_repaint();
	}
}
