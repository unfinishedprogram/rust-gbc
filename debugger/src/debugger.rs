use crate::components::{
	AudioVisualizer, Disassembler, JoypadInput, MemoryImage, MemoryView, RomLoader, Screen,
	VramView,
	run_controller::{self, RunController},
	show_system_info,
	timeline::Timeline,
};
use egui::{CentralPanel, Panel, Style, TextStyle, Window};

use gameboy::Gameboy;

#[derive(Default)]
pub struct Debugger {
	gameboy: Gameboy,
	screen: Screen,
	run_controller: RunController,
	rom_loader: RomLoader,
	joypad: JoypadInput,

	// Debugging Windows
	memory_image: MemoryImage,
	memory_image_enabled: bool,

	vram_view: VramView,
	vram_view_enabled: bool,

	memory_view: MemoryView,
	memory_view_enabled: bool,

	disassembler: Disassembler,
	disassembler_enabled: bool,

	audio_visualizer: AudioVisualizer,
	audio_visualizer_enabled: bool,

	timeline: Timeline,
	timeline_enable: bool,
}

impl Debugger {
	pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		Default::default()
	}

	// Returns true if breakpoint was hit
	fn step_gb(&mut self) -> bool {
		self.timeline.step_gameboy(&mut self.gameboy);
		self.disassembler.should_break(&self.gameboy)
	}
}

impl eframe::App for Debugger {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
		self.gameboy.set_controller_state(self.joypad.update(ui));

		ui.set_style(Style {
			override_text_style: Some(TextStyle::Monospace),
			..Default::default()
		});

		Panel::top("top").show(ui, |ui| {
			ui.horizontal(|ui| {
				if let Some(action) = self
					.run_controller
					.draw(ui, self.disassembler.should_break(&self.gameboy))
				{
					match action {
						run_controller::Action::StepFrame => {
							let start = self.gameboy.ppu.frame;
							while self.gameboy.ppu.frame == start {
								if self.step_gb() {
									break;
								}
							}
						}
						run_controller::Action::Step(cycles) => {
							for _ in 0..cycles {
								if self.step_gb() {
									break;
								}
							}
						}
						run_controller::Action::NextInterrupt => {
							while !(self.gameboy.cpu_state.interrupt_pending()
								&& (self.gameboy.cpu_state.ime() || self.gameboy.cpu_state.halted))
							{
								if self.step_gb() {
									break;
								}
							}
						}
						run_controller::Action::SkipBios => {
							while self.gameboy.booting {
								if self.step_gb() {
									break;
								}
							}
						}
						run_controller::Action::HDMAStart => {
							let mut steps = 0;
							while self.gameboy.dma_controller.read_hdma5() & 0b10000000 != 0
								&& steps < 10000000
							{
								steps += 1;
								if self.step_gb() {
									break;
								}
							}
						}
						run_controller::Action::OAMDmaStart => todo!(),
					}
				}

				let rom_loaded = self.rom_loader.draw(ui, &mut self.gameboy);
				if rom_loaded {
					self.timeline.reset();
				}

				ui.checkbox(&mut self.timeline_enable, "Timeline");
				ui.checkbox(&mut self.memory_image_enabled, "Memory Image");
				ui.checkbox(&mut self.vram_view_enabled, "Vram View");
				ui.checkbox(&mut self.memory_view_enabled, "Memory View");
				ui.checkbox(&mut self.disassembler_enabled, "Instruction View");
				ui.checkbox(&mut self.audio_visualizer_enabled, "Audio Visualizer");
			});
		});

		let screen_buffer = self.gameboy.ppu.lcd.front_buffer();

		Panel::right("right").show(ui, |ui| show_system_info(&self.gameboy, ui));

		CentralPanel::default().show(ui, |ui| self.screen.draw(ui, screen_buffer));

		if self.disassembler_enabled {
			Window::new("Instructions").show(ui, |ui| self.disassembler.draw(&self.gameboy, ui));
		}

		if self.memory_view_enabled {
			Window::new("Memory").show(ui, |ui| self.memory_view.draw(&self.gameboy, ui));
		}
		if self.memory_image_enabled {
			Window::new("MemImage").show(ui, |ui| self.memory_image.draw(&self.gameboy, ui));
		}

		if self.vram_view_enabled {
			Window::new("VramView").show(ui, |ui| self.vram_view.draw(&self.gameboy, ui));
		}

		if self.timeline_enable {
			Window::new("Timeline").show(ui, |ui| self.timeline.draw(&mut self.gameboy, ui));
		}

		if self.audio_visualizer_enabled {
			Window::new("Audio Visualizer")
				.show(ui, |ui| self.audio_visualizer.draw(&mut self.gameboy, ui));
		}

		ui.request_repaint();
	}
}
