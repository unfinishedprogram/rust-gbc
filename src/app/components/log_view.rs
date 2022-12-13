use egui::Ui;
use egui_extras::{Size, TableBuilder};

pub fn draw_logs(ui: &mut Ui, logs: &Vec<String>) {
	ui.label(format!("Logs: {:}", logs.len()));
	TableBuilder::new(ui)
		.scroll(true)
		.striped(true)
		.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
		.stick_to_bottom(true)
		.column(Size::Remainder {
			range: (0.0, 500.0),
		})
		.body(|body| {
			body.rows(18.0, logs.len(), |index, mut row| {
				row.col(|ui| {
					ui.label(logs[index].clone());
				});
			});
		});
}
