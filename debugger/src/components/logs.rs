use egui::Ui;

pub struct Logs;
impl Logs {
	pub fn draw(_ui: &mut Ui) {
		// let mut debugger = DEBUGGER.lock().unwrap();

		// if ui.button("Clear Logs").clicked() {
		// 	debugger.events.clear();
		// }

		// TableBuilder::new(ui)
		// 	.vscroll(true)
		// 	.striped(true)
		// 	.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
		// 	.stick_to_bottom(true)
		// 	.column(Column::auto())
		// 	.body(|body| {
		// 		body.rows(18.0, debugger.events.len(), |index, mut row| {
		// 			row.col(|ui| {
		// 				ui.label(format!("{:?}", debugger.events[index]));
		// 			});
		// 		});
		// 	});
	}
}
