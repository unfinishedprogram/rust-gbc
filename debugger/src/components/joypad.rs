use egui::{Context, Key};
use gameboy::joypad::JoypadState;

#[derive(Default)]
pub struct JoypadInput {
	controller_state: JoypadState,
}

impl JoypadInput {
	pub fn update(&mut self, ctx: &Context) -> &JoypadState {
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
