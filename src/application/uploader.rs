use gloo::console::log;
use gloo::events::{EventListener, EventListenerOptions};
use gloo::file::{callbacks::read_as_bytes, File};
use gloo::utils::document;
use wasm_bindgen::JsCast;

use super::APPLICATION;

pub fn on_file_drop() {
	EventListener::new_with_options(
		&document().get_element_by_id("screen").unwrap(),
		"drop",
		EventListenerOptions::enable_prevent_default(),
		|e| {
			e.prevent_default();
			let e = e.dyn_ref::<web_sys::DragEvent>().unwrap();
			if let Some(data_transferer) = e.data_transfer() {
				if let Some(files) = data_transferer.files() {
					if let Some(file) = files.get(0) {
						let file = File::from(file);
						let name = file.name();
						log!(&name);

						APPLICATION.with_borrow_mut(move |app| {
							app._file_reader = Some(read_as_bytes(&file, move |res| {
								log!("here");
								APPLICATION.with_borrow_mut(move |app| {
									app.load_rom(&res.unwrap(), name);
									app._file_reader = None;
								})
							}));
						});
					}
				}
			}
		},
	)
	.forget();

	EventListener::new_with_options(
		&document().get_element_by_id("screen").unwrap(),
		"dragover",
		EventListenerOptions::enable_prevent_default(),
		|e| {
			e.prevent_default();
		},
	)
	.forget()
}
