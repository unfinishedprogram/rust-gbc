use egui::{DragValue, Ui};
use gameboy::Gameboy;

use crate::components::validated_input::ValidatedInput;

use super::TStates;

pub struct CheckpointManager {
	checkpoints: Vec<Gameboy>,
	time: TStates,

	t_state_input: ValidatedInput<u64>,
	m_state_input: ValidatedInput<u64>,
	second_input: ValidatedInput<f64>,
	frame_input: ValidatedInput<f64>,

	unit: UnitType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnitType {
	MState,
	TState,
	Second,
	Frame,
}

impl Default for CheckpointManager {
	fn default() -> Self {
		Self {
			unit: UnitType::Second,
			checkpoints: Default::default(),
			time: Default::default(),
			t_state_input: ValidatedInput::new(|s| s.parse::<u64>().map_err(|e| e.to_string()))
				.default("0")
				.label("T-State"),
			m_state_input: ValidatedInput::new(|s| s.parse::<u64>().map_err(|e| e.to_string()))
				.default("0")
				.label("M-State"),
			second_input: ValidatedInput::new(|s| s.parse::<f64>().map_err(|e| e.to_string()))
				.default("0.0")
				.label("Second"),
			frame_input: ValidatedInput::new(|s| s.parse::<f64>().map_err(|e| e.to_string()))
				.default("0.0")
				.label("Frame"),
		}
	}
}

impl CheckpointManager {
	pub fn reset(&mut self) {
		self.checkpoints.clear();
	}

	pub fn save(&mut self, state: &Gameboy) {
		self.checkpoints.push(state.clone());
		self.checkpoints.sort_by(|a, b| a.t_states.cmp(&b.t_states))
	}

	pub fn load(&mut self, time: impl Into<TStates>) -> Option<Gameboy> {
		let t_states = time.into().t_states();

		for cp in &self.checkpoints {
			if cp.t_states <= t_states {
				let mut gb = cp.clone();

				while gb.t_states < t_states {
					gb.step();
					if self.closest_distance(gb.t_states) > TStates::from_seconds(0.5).t_states() {
						self.save(&gb);
					}
				}

				return Some(gb);
			}
		}
		None
	}

	pub fn count(&self) -> usize {
		self.checkpoints.len()
	}

	pub fn closest_distance(&self, t_state: u64) -> u64 {
		let mut min_dist = u64::MAX;

		for cp in self.checkpoints.iter().filter(|v| v.t_states <= t_state) {
			min_dist = min_dist.min(t_state - cp.t_states)
		}

		min_dist
	}

	pub fn size(&self) -> usize {
		self.checkpoints.len() * std::mem::size_of::<Gameboy>()
	}

	pub fn draw(&mut self, gameboy: &mut Gameboy, ui: &mut Ui) {
		// let t_state_unit = TimeUnit::TStates(self.time.t_states());
		// let m_state_unit = TimeUnit::MStates(MStates::from(self.time.t_states()));
		// let frame_unit = TimeUnit::Frames(Frames::from(self.time.t_states()));
		// let second_unit = TimeUnit::Seconds(Seconds::from(self.time.t_states()));

		egui::ComboBox::from_label("Unit")
			.selected_text(format!("{:?}", self.unit))
			.show_ui(ui, |ui| {
				ui.set_width(100.0);
				ui.selectable_value(&mut self.unit, UnitType::TState, "TState");
				ui.selectable_value(&mut self.unit, UnitType::MState, "MState");
				ui.selectable_value(&mut self.unit, UnitType::Frame, "Frame");
				ui.selectable_value(&mut self.unit, UnitType::Second, "Second");
			});

		self.time = match self.unit {
			UnitType::MState => {
				if ui.add(&mut self.m_state_input).changed() {
					self.m_state_input.value().map(TStates::from_m_states)
				} else {
					None
				}
			}
			UnitType::TState => {
				if ui.add(&mut self.t_state_input).changed() {
					self.m_state_input.value().map(TStates::from_t_states)
				} else {
					None
				}
			}
			UnitType::Second => {
				if ui.add(&mut self.second_input).changed() {
					self.second_input.value().map(TStates::from_seconds)
				} else {
					None
				}
			}
			UnitType::Frame => {
				if ui.add(&mut self.frame_input).changed() {
					self.frame_input.value().map(TStates::from_frames)
				} else {
					None
				}
			}
		}
		.unwrap_or(self.time);

		ui.label(format!("Count: {}", self.count()));
		ui.label(format!("Size : {}kb", self.size() / 1024));
		ui.label(format!("Cycle: {}", gameboy.t_states));

		if self.closest_distance(gameboy.t_states) > TStates::from_seconds(0.5).t_states() {
			self.save(gameboy);
		}

		if ui.button("Reset").clicked() {
			self.reset();
		}

		if ui.button("Load").clicked() {
			if let Some(gb) = self.load(self.time) {
				*gameboy = gb
			}
		}
	}
}
