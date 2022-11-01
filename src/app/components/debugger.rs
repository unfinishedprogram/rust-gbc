mod breakpoint_manager;
pub mod cpu_status;
mod debug_draw;
mod memory_view;

use super::{logger, BufferView};
use crate::{app::drawable::DrawableMut, emulator::state::EmulatorState};
use breakpoint_manager::BreakpointManager;
use debug_draw::*;
use egui::Ui;
use memory_view::MemoryView;

enum DebuggerState {
	Playing,
	Paused,
}

pub struct Debugger {
	state: DebuggerState,
	cycle: u64,
	breakpoint_manager: BreakpointManager,
	memory_view: MemoryView,
	vram_view: BufferView,
	window_view: BufferView,
	pub emulator_state: EmulatorState,
	pub run: bool,
}

impl Default for Debugger {
	fn default() -> Self {
		Self {
			run: false,
			cycle: 0,
			emulator_state: EmulatorState::default().init(),
			state: DebuggerState::Paused,
			breakpoint_manager: BreakpointManager::default(),
			memory_view: MemoryView::default(),
			vram_view: BufferView::new("VRAM", (16 * 8, 24 * 8)),
			window_view: BufferView::new("Window", (256, 256)),
		}
	}
}

impl Debugger {
	pub fn draw(&mut self, ui: &mut Ui) {
		debug_draw_tile_data(&self.emulator_state, &mut self.vram_view.pixel_buffer);
		debug_draw_window_data(&self.emulator_state, &mut self.window_view.pixel_buffer);

		self.vram_view.draw_window(ui, "Vram");
		self.window_view.draw_window(ui, "Window");

		ui.label(format!("Cycle: {:}", self.cycle));

		self.breakpoint_manager.draw(ui);

		ui.separator();

		self.memory_view
			.draw(ui, &mut self.emulator_state, &mut self.breakpoint_manager);
	}

	pub fn back(&mut self) {
		// if let Some(state) = self.save_states.pop() {
		// 	self.cycle -= 1;
		// 	self.emulator_state = state;
		// }
	}

	pub fn step(&mut self) {
		if !self.run {
			return;
		}
		loop {
			self.cycle += 1;
			self.emulator_state.step();
			if self.emulator_state.cpu_state.t_states == 0 {
				break;
			}
		}
	}

	pub fn run_until_break(&mut self, state: &mut EmulatorState) {
		if self
			.breakpoint_manager
			.break_on(state.cpu_state.registers.pc)
		{}
		todo!();
	}
}
