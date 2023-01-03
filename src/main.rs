#![feature(local_key_cell_methods)]

use gbc_emu::application::{setup_listeners, APPLICATION};

fn main() {
	APPLICATION.with_borrow_mut(|app| app.start());
	setup_listeners();
}
