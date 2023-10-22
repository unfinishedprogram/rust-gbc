use egui::{Context, Key};
use gameboy::controller::ControllerState;

#[derive(Default)]
pub struct JoypadInput {
	controller_state: ControllerState,
}

impl JoypadInput {
	pub fn update(&mut self, ctx: &Context) -> &ControllerState {
		ctx.input(|input| {
			let events = input.events.clone();
			for event in events {
				if let egui::Event::Key { key, pressed, .. } = event {
					match key {
						Key::ArrowDown => self.controller_state.down = pressed,
						Key::ArrowLeft => self.controller_state.left = pressed,
						Key::ArrowRight => self.controller_state.right = pressed,
						Key::ArrowUp => self.controller_state.up = pressed,
						Key::Z => self.controller_state.a = pressed,
						Key::X => self.controller_state.b = pressed,
						Key::Space => self.controller_state.select = pressed,
						Key::Enter => self.controller_state.start = pressed,
						_ => {}
					}
				}
			}
		});

		&self.controller_state
	}
}
