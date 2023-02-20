use gloo::console::log;
use gloo::utils::document;

use super::{events::add_click_listener, uploader::setup_upload_listeners, APPLICATION};

pub fn setup_listeners() {
	setup_upload_listeners();
	add_click_listener("#toggle_play", |_e| {
		log!("Toggling!");
		let elm = document().get_element_by_id("toggle_play").unwrap();

		APPLICATION.with_borrow_mut(|app| {
			elm.set_inner_html(&format!("{}", app.running_state));
			app.toggle_play();
		})
	});

	add_click_listener("#step", |_e| {
		APPLICATION.with_borrow_mut(|app| {
			app.step_single();
		})
	})
}
