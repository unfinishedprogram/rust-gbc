use crate::util::file_types::Entry;
use egui::Ui;

pub fn file_selector(ui: &mut Ui, files: &Entry, on_select: &mut impl FnMut(&str)) {
	match files {
		Entry::File(name, path) => {
			if ui.button(name).clicked() {
				ui.close_menu();
				on_select(path.as_str());
			}
		}
		Entry::Dir(name, entries) => {
			ui.menu_button(name, |ui| {
				for entry in entries.values() {
					file_selector(ui, entry, on_select);
				}
			});
		}
	}
}
