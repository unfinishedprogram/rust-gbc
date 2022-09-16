use egui::Context;
use egui_extras::{Size, TableBuilder};

pub fn log_view(ctx: &Context, logs: &Vec<(u16, String)>) {
	egui::Window::new("Logs")
		.resizable(true)
		.vscroll(true)
		.show(ctx, |ui| {
			ui.vertical_centered(|ui| {
				let table = TableBuilder::new(ui)
					.striped(true)
					.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
					.column(Size::Absolute {
						initial: (20.0),
						range: (10.0, 20.0),
					})
					.column(Size::Absolute {
						initial: (400.0),
						range: (10.0, 400.0),
					})
					.scroll(true)
					.stick_to_bottom(true);

				table.body(|body| {
					body.rows(16.0, logs.len(), |row_index, mut row| {
						row.col(|ui| {
							ui.monospace(format!("{:X}", &logs[row_index].0));
						});
						row.col(|ui| {
							ui.monospace(&logs[row_index].1);
						});
					});
				});
			});
		});
}
