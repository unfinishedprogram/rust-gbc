mod breakpoint_manager;
mod debug_draw;
mod memory_view;
pub mod status;

use super::{logger, BufferView};
use crate::{
	app::drawable::DrawableMut,
	emulator::{memory_mapper::MemoryMapper, state::EmulatorState},
};
use breakpoint_manager::BreakpointManager;
use debug_draw::*;
use egui::Ui;
use memory_view::MemoryView;

enum DebuggerState {
	Running,
	Paused,
}

pub struct Debugger {
	state: DebuggerState,
	cycle: u64,
	breakpoint_manager: BreakpointManager,
	memory_view: MemoryView,
	vram_view: BufferView,
	window_view: BufferView,
	serial_output: Vec<char>,
	frame_time: String,
	pub emulator_state: EmulatorState,
}

impl Default for Debugger {
	fn default() -> Self {
		Self {
			cycle: 0,
			frame_time: "".to_string(),
			serial_output: vec![],
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
		ui.label(format!(
			"Cycle: {:}",
			format!("{:b}", self.emulator_state.read(0xFF02)) // self.serial_output.clone().into_iter().collect::<String>()
		));
		ui.label(format!("Frametime: {:}ms", self.frame_time));

		self.breakpoint_manager.draw(ui);

		ui.separator();

		self.memory_view
			.draw(ui, &mut self.emulator_state, &mut self.breakpoint_manager);
	}

	pub fn start(&mut self) {
		self.state = DebuggerState::Running;
	}

	pub fn pause(&mut self) {
		self.state = DebuggerState::Paused;
	}

	pub fn toggle_state(&mut self) {
		match self.state {
			DebuggerState::Running => self.pause(),
			DebuggerState::Paused => self.start(),
		}
	}

	pub fn do_serial(&mut self) {
		logger::debug(format!("{:X}", self.emulator_state.read(0xFF02)));
		if self.emulator_state.read(0xFF02) >> 7 == 1 {
			self.serial_output
				.push(self.emulator_state.read(0xFF01) as char)
		}
	}

	pub fn step(&mut self) {
		match self.state {
			DebuggerState::Paused => {}
			DebuggerState::Running => {
				// self.cycle += 1;
				let now = instant::Instant::now();
				let start = self.emulator_state.cycle;
				while self.emulator_state.cycle - start < 69905 {
					self.emulator_state.step();
					self.do_serial();
				}
				self.cycle = self.emulator_state.cycle;
				self.frame_time = format!("{}", now.elapsed().as_millis());
				// if self.emulator_state.cpu_state.t_states == 0 {
				// break;
				// }
			}
		}
	}
}
