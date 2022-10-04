mod breakpoint_manager;
mod cpu_status;
mod memory_view;

use crate::{app::drawable::DrawableMut, emulator::Emulator};
use breakpoint_manager::BreakpointManager;
use cpu_status::draw_cpu_status;
use egui::Ui;
use memory_view::MemoryView;

use super::logger::Logger;

pub struct Debugger {
	breakpoint_manager: BreakpointManager,
	memory_view: MemoryView,
}

impl Default for Debugger {
	fn default() -> Self {
		Self {
			breakpoint_manager: BreakpointManager::default(),
			memory_view: MemoryView::default(),
		}
	}
}

impl Debugger {
	pub fn draw(&mut self, emulator: &mut Emulator, logger: &mut Logger, ui: &mut Ui) {
		self.breakpoint_manager.draw(ui);
		ui.separator();
		self.memory_view
			.draw(ui, emulator, &mut self.breakpoint_manager);
	}
	pub fn apply(&mut self, emulator: &mut Emulator) {}
}
