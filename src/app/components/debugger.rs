mod breakpoint_manager;
mod cpu_status;
mod memory_view;

use crate::{app::drawable::DrawableMut, emulator::Emulator};
use breakpoint_manager::BreakpointManager;
use egui::Ui;
use memory_view::MemoryView;

use super::logger;

pub enum DebuggerState {
	Playing,
	Paused,
}

pub struct Debugger {
	breakpoint_manager: BreakpointManager,
	state: DebuggerState,
	memory_view: MemoryView,
}

impl Default for Debugger {
	fn default() -> Self {
		Self {
			state: DebuggerState::Paused,
			breakpoint_manager: BreakpointManager::default(),
			memory_view: MemoryView::default(),
		}
	}
}

impl Debugger {
	pub fn draw(&mut self, emulator: &mut Emulator, ui: &mut Ui) {
		self.breakpoint_manager.draw(ui);
		ui.separator();
		self.memory_view
			.draw(ui, emulator, &mut self.breakpoint_manager);
	}

	pub fn apply(&mut self, emulator: &mut Emulator) {
		if self.breakpoint_manager.break_on(emulator.cpu.registers.pc) {}
	}

	pub fn step(&mut self, t_states: u32, emulator: &mut Emulator) {
		match self.state {
			DebuggerState::Paused => {}
			DebuggerState::Playing => {
				for _ in 0..t_states {
					emulator.step();
					if self.breakpoint_manager.break_on(emulator.cpu.registers.pc) {
						self.state = DebuggerState::Paused;
						logger::warn(format!("Paused at: {}", emulator.cpu.registers.pc));
						break;
					}
				}
			}
		}
	}

	pub fn run_until_break(&mut self, emulator: &mut Emulator) {
		if self.breakpoint_manager.break_on(emulator.cpu.registers.pc) {}
		emulator.step();
		self.apply(emulator);
	}
}
