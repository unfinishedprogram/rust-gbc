use std::cell::RefCell;

use egui::Ui;
use gameboy::Gameboy;

#[derive(Default)]
pub struct RomLoader {
	url: String,
	error_msg: Option<String>,
}

pub struct RomResource {
	response: ehttp::Response,
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Entry {
	File { path: String },
	Dir { path: String, entries: Vec<Entry> },
}

thread_local! {
	pub static ROMS : Entry = serde_json::from_str(std::include_str!("../../../roms/roms.json")).unwrap();
	pub static LOAD_RESULT:RefCell<Option<Result<RomResource, String>>> = const { RefCell::new(None) };
}

fn recursive_dir(ui: &mut Ui, url: &mut String, entry: &Entry) -> bool {
	fn file_name(path: &str) -> &str {
		path.split('/').next_back().unwrap()
	}
	let mut updated = false;
	match entry {
		Entry::File { path } => {
			ui.set_min_width(300.0);
			if ui.button(file_name(path)).clicked() {
				*url = path.clone();
				updated = true;
				ui.close_menu();
			}
		}
		Entry::Dir { path, entries } => {
			ui.menu_button(file_name(path), |ui| {
				for entry in entries {
					updated |= recursive_dir(ui, url, entry);
				}
			});
		}
	}

	updated
}

impl RomLoader {
	pub fn load_rom(&mut self, ui: &Ui) {
		let ctx = ui.ctx().clone();

		let request = ehttp::Request::get(&self.url);

		ehttp::fetch(request, move |response| {
			let resource = response.map(|response| RomResource { response });
			LOAD_RESULT.with(|r| *r.borrow_mut() = Some(resource));
			ctx.request_repaint();
		});
	}

	pub fn draw(&mut self, ui: &mut Ui, gameboy: &mut Gameboy) {
		if recursive_dir(ui, &mut self.url, &ROMS.with(|r| r.clone())) {
			self.load_rom(ui);
		}

		match LOAD_RESULT.with(|r| r.borrow_mut().take()) {
			Some(Ok(resource)) => {
				*gameboy = Gameboy::cgb();
				gameboy.load_rom(&resource.response.bytes, None);
			}
			Some(Err(error)) => {
				let msg = if error.is_empty() { "Error" } else { &error };
				self.error_msg = Some(msg.to_owned());
			}
			None => {}
		}

		if let Some(error) = &self.error_msg {
			ui.label(error);
		}
	}
}
