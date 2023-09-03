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
	pub static ROMS : Entry = serde_json::from_str(std::include_str!("../../../roms.json")).unwrap();
	pub static LOAD_RESULT:RefCell<Option<Result<RomResource, String>>> = RefCell::new(None);
}

fn recursive_dir(ui: &mut Ui, url: &mut String, entry: &Entry) {
	match entry {
		Entry::File { path } => {
			if ui.button(path).clicked() {
				*url = path.clone();
				ui.close_menu();
			}
		}
		Entry::Dir { path, entries } => {
			ui.menu_button(path, |ui| {
				for entry in entries {
					recursive_dir(ui, url, entry);
				}
			});
		}
	}
}

impl RomLoader {
	pub fn draw(&mut self, ui: &mut Ui, gameboy: &mut Gameboy) {
		ui.menu_button("roms", |ui| {
			recursive_dir(ui, &mut self.url, &ROMS.with(|r| r.clone()));
		});

		ui.text_edit_singleline(&mut self.url);

		if ui.button("load").clicked() {
			let ctx = ui.ctx().clone();

			let request = ehttp::Request::get(&self.url);

			ehttp::fetch(request, move |response| {
				let resource = response.map(|response| RomResource { response });
				LOAD_RESULT.with(|r| *r.borrow_mut() = Some(resource));
				ctx.request_repaint();
			});
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
			None => {
				ui.spinner();
			}
		}

		if let Some(error) = &self.error_msg {
			ui.label(error);
		}
	}
}
