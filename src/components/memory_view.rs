use crate::cpu::Cpu;
use egui::{Context, Rgba, RichText};
use egui_extras::{Size, TableBuilder};

pub fn memory_view(ctx: &Context, cpu: &Cpu) {
	egui::Window::new("Memory")
		.resizable(true)
		.vscroll(true)
		.show(ctx, |ui| {
			ui.vertical_centered(|ui| {
				let table = TableBuilder::new(ui)
					.striped(true)
					.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
					.columns(
						Size::Absolute {
							initial: (15.0),
							range: (10.0, 20.0),
						},
						32,
					)
					.scroll(true);

				table.body(|body| {
					body.rows(16.0, 4096 / 32, |row_index, mut row| {
						for i in 0..32 {
							row.col(|ui| {
								ui.colored_label(
									match row_index * 32 + i == cpu.registers.pc as usize {
										true => Rgba::from_rgb(255.0, 0.0, 0.0),
										false => Rgba::from_rgb(255.0, 255.0, 255.0),
									},
									RichText::new(format!("{:X}", cpu.memory[row_index * 32 + i]))
										.monospace(),
								);
							});
						}
					});
				});
			});
		});
}
