mod checkpoint_manager;
mod t_states;

pub use checkpoint_manager::CheckpointManager;
use egui::{Color32, Pos2, Rect, Sense, Vec2};
use gameboy::{Gameboy, joypad::JoypadState};
use sm83::{Instruction, SM83};
pub use t_states::TStates;

pub struct TimelineEvent {
	pub t_states: u64,
	pub event_type: EventType,
}

impl TimelineEvent {
	pub fn new(t_states: u64, event_type: EventType) -> Self {
		Self {
			t_states,
			event_type,
		}
	}
}

pub struct Checkpoint {
	pub t_states: u64,
	pub state: Box<Gameboy>,
}
pub enum EventType {
	FrameEnd,
	InstructionExecuted(Instruction),
	Input { state: JoypadState },
}

#[derive(Default)]
pub struct Timeline {
	checkpoints: Vec<Checkpoint>,
	events: Vec<TimelineEvent>,
	latest_t_states: u64,

	view_start: u64,
	view_end: u64,
	show_single_frame: bool,
}

impl Timeline {
	pub fn draw(&mut self, gameboy: &mut Gameboy, ui: &mut egui::Ui) {
		ui.label(format!("T-States: {}", gameboy.t_states));
		ui.label(format!("Latest T-States: {}", self.latest_t_states));
		ui.label(format!("Events: {}", self.events.len()));

		self.show_single_frame = true;

		if self.show_single_frame {
			let frame_start = self
				.events
				.iter()
				.rev()
				.filter(|e| matches!(e.event_type, EventType::FrameEnd))
				.skip(1)
				.map(|e| e.t_states)
				.next()
				.unwrap_or(0);

			let frame_end = self
				.events
				.iter()
				.rev()
				.filter(|e| matches!(e.event_type, EventType::FrameEnd))
				.map(|e| e.t_states)
				.next()
				.unwrap_or(0);

			self.draw_timeline(ui, frame_start, frame_end);
		} else {
			self.draw_timeline(ui, self.view_start, self.view_end);
		}
	}

	fn draw_timeline(&mut self, ui: &mut egui::Ui, start: u64, end: u64) {
		const TIMELINE_HEIGHT: f32 = 200.0;

		let (response, painter) = ui.allocate_painter(
			Vec2::new(ui.available_width(), TIMELINE_HEIGHT),
			Sense::hover(),
		);
		let width = response.rect.width();
		let pixels_per_t_state = width / (end - start) as f32;

		let top_left: Vec2 = response.rect.left_top().to_vec2();

		for event in self.events_between(start, end).iter() {
			let t_states_offset = event.t_states.saturating_sub(start);

			let x = (t_states_offset as f32 * pixels_per_t_state).round();
			let color = match event.event_type {
				EventType::FrameEnd => Color32::from_rgb(255, 255, 255),
				EventType::InstructionExecuted(_) => Color32::from_rgb(255, 0, 255),
				EventType::Input { state: _ } => Color32::from_rgb(255, 255, 255),
			};

			painter.line_segment(
				[
					Pos2::new(x, 0.0) + top_left,
					Pos2::new(x, response.rect.height()) + top_left,
				],
				(1.0, color),
			);
		}
	}

	fn events_between(&self, start: u64, end: u64) -> &[TimelineEvent] {
		let start_index = self
			.events
			.binary_search_by_key(&start, |e| e.t_states)
			.unwrap_or_else(|x| x);
		let end_index = self
			.events
			.binary_search_by_key(&end, |e| e.t_states)
			.unwrap_or_else(|x| x);

		&self.events[start_index..end_index]
	}

	pub fn add_event(&mut self, event: TimelineEvent) {
		self.events.push(event);
	}

	pub fn step_gameboy(&mut self, gameboy: &mut Gameboy) -> Option<Instruction> {
		let previous_frame = gameboy.ppu.frame;
		let instruction = gameboy.step();

		if gameboy.ppu.frame != previous_frame {
			self.add_event(TimelineEvent::new(gameboy.t_states, EventType::FrameEnd));
		}

		if let Some(Instruction::INT) = instruction {
			self.add_event(TimelineEvent::new(
				gameboy.t_states,
				EventType::InstructionExecuted(Instruction::INT),
			));
		}

		self.view_end = gameboy.t_states;
		self.view_start = self.view_end.saturating_sub(100_000_00);

		instruction
	}
}
