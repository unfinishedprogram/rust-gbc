use egui::Ui;
use egui_extras::{Size, TableBuilder};

pub fn log_view(ui: &mut Ui, logs: &Vec<(u16, String)>) {
	ui.heading("Call Log");
	ui.separator();
	TableBuilder::new(ui)
		.striped(true)
		.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
		.scroll(true)
		.stick_to_bottom(true)
		.column(Size::Remainder {
			range: (0.0, 500.0),
		})
		.body(|body| {
			body.rows(16.0, logs.len(), |row_index, mut row| {
				row.col(|ui| {
					ui.monospace(format!(
						"{:04X} : {}",
						&logs[row_index].0, &logs[row_index].1
					));
				});
			});
		});
}
