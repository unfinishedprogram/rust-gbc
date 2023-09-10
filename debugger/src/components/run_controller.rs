use egui::Ui;

pub enum Action {
	StepFrame,
	NextInterrupt,
	PPUStateChange,
	HDMAStart,
	OAMDmaStart,
	Step(u64),
}

#[derive(Default)]
pub enum RunningState {
	#[default]
	Paused,
	Broke,
	Running,
}

#[derive(Default)]
pub struct RunController {
	state: RunningState,
	cycles_per_frame: u64,
}

impl RunController {
	pub fn draw(&mut self, ui: &mut Ui) -> Option<Action> {
		ui.horizontal(|ui| {
			let button_txt = match self.state {
				RunningState::Paused => "▶",
				RunningState::Broke => "▶",
				RunningState::Running => "⏹",
			};

			if ui.button(button_txt).on_hover_text("Toggle Play").clicked() {
				self.state = match self.state {
					RunningState::Paused => RunningState::Running,
					RunningState::Broke => RunningState::Running,
					RunningState::Running => RunningState::Paused,
				}
			}

			let mut action: Option<Action> = None;
			ui.add(egui::DragValue::new(&mut self.cycles_per_frame));

			if ui.button("⏵").on_hover_text("Single Step").clicked() {
				action = Some(Action::Step(1));
			}

			if ui.button("⏩").on_hover_text("Step Frame").clicked() {
				action = Some(Action::StepFrame);
			}

			if ui.button("NextInterrupt").clicked() {
				action = Some(Action::NextInterrupt);
			}

			if ui.button("PPUStateChange").clicked() {
				action = Some(Action::PPUStateChange);
			}

			if ui.button("HDMAStart").clicked() {
				action = Some(Action::HDMAStart);
			}

			if ui.button("OAMDmaStart").clicked() {
				action = Some(Action::OAMDmaStart);
			}

			if action.is_some() {
				return action;
			}

			match self.state {
				RunningState::Paused => None,
				RunningState::Broke => None,
				RunningState::Running => Some(Action::Step(self.cycles_per_frame)),
			}
		})
		.inner
	}
}
