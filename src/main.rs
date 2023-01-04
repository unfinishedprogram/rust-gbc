#![feature(local_key_cell_methods)]

use gbc_emu::application::{setup_listeners, APPLICATION};

fn main() {
	APPLICATION.with_borrow_mut(|app| {
		app.load_rom(
			include_bytes!("../roms/games/Kirby's Dream Land (USA, Europe).gb"),
			"Kirby".to_string(),
		);
		app.start();
	});
	setup_listeners();
}
