use std::{
	io::{stdout, Write},
	thread::{self},
	time::{Duration, Instant},
};

use climg::image_builder::{ImageBuilder, ImageBuilderConfig};

use crossterm::{
	event::{
		KeyCode, KeyEvent, KeyEventKind, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
		PushKeyboardEnhancementFlags,
	},
	execute,
};

use gameboy::{joypad::JoypadState, Gameboy};

fn main() {
	let mut stdout = stdout();
	execute!(
		stdout,
		PushKeyboardEnhancementFlags(
			KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
				| KeyboardEnhancementFlags::REPORT_EVENT_TYPES
		)
	)
	.unwrap();

	let mut gb = Gameboy::default();

	gb.load_rom(
		include_bytes!("../../../rust-gbc/roms/games/Super Mario Bros. Deluxe.gbc"),
		None,
	);

	let config = ImageBuilderConfig {
		skip_unchanged: true,
	};

	let mut render_builder = ImageBuilder::new(160, 144, config);
	let mut controller_state = JoypadState::default();

	'outer: loop {
		crossterm::terminal::enable_raw_mode().unwrap();

		let start = Instant::now();
		while let Ok(true) = crossterm::event::poll(Duration::ZERO) {
			let event = crossterm::event::read().unwrap();
			if let crossterm::event::Event::Key(KeyEvent {
				code,
				modifiers: _,
				kind,
				state: _,
			}) = event
			{
				let is_down = match kind {
					KeyEventKind::Press => true,
					KeyEventKind::Release => false,
					KeyEventKind::Repeat => true,
				};
				match code {
					KeyCode::Esc => break 'outer,
					KeyCode::Char('q') => break 'outer,
					KeyCode::Left => controller_state.left = is_down,
					KeyCode::Right => controller_state.right = is_down,
					KeyCode::Up => controller_state.up = is_down,
					KeyCode::Down => controller_state.down = is_down,
					KeyCode::Tab => controller_state.select = is_down,
					KeyCode::Char('z') => controller_state.a = is_down,
					KeyCode::Char('x') => controller_state.b = is_down,
					_ => {}
				}
			}
		}

		gb.set_controller_state(&controller_state);

		let start_frame = gb.ppu.frame;

		while gb.ppu.frame == start_frame {
			gb.step();
		}
		let screen = gb.ppu.lcd.front_buffer();
		render_builder.draw_img(screen);
		let output = render_builder.build();
		let mut stdout = std::io::stdout();
		stdout.write_all(output.as_bytes()).unwrap();
		stdout.flush().unwrap();
		while start.elapsed() < (Duration::from_secs(1) / 60) {
			thread::sleep(Duration::from_micros(100))
		}
	}

	crossterm::terminal::disable_raw_mode().unwrap();
	execute!(stdout, PopKeyboardEnhancementFlags).unwrap();
}
