mod breakpoint_manager;
mod cpu_status;
mod debug_draw;
mod memory_view;

use super::{logger, BufferView};
use crate::app::drawable::DrawableMut;
use crate::emulator::Emulator;
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
	breakpoint_manager: BreakpointManager,
	memory_view: MemoryView,
	vram_view: BufferView,
	window_view: BufferView,
}

impl Default for Debugger {
	fn default() -> Self {
		Self {
			state: DebuggerState::Paused,
			breakpoint_manager: BreakpointManager::default(),
			memory_view: MemoryView::default(),
			vram_view: BufferView::new("VRAM", (256, 256)),
			window_view: BufferView::new("Window", (256, 256)),
		}
	}
}

impl Debugger {
	pub fn draw(&mut self, emulator: &mut Emulator, ui: &mut Ui) {
		debug_draw_tile_data(&emulator.memory, &mut self.vram_view.pixel_buffer);
		debug_draw_window_data(&emulator.memory, &mut self.window_view.pixel_buffer);

		self.vram_view.draw_window(ui, "Vram");
		self.window_view.draw_window(ui, "Window");

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
