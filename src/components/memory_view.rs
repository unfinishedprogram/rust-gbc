use crate::cpu::Cpu;
use egui::{Color32, Label, Rgba, RichText, Sense, Ui};
use egui_extras::{Size, TableBuilder};

pub struct MemoryViewState {
	selected: Option<u16>,
	hovering: Option<u16>,
}

impl Default for MemoryViewState {
	fn default() -> Self {
		Self {
			selected: None,
			hovering: None,
		}
	}
}

pub fn memory_view(ui: &mut Ui, cpu: &Cpu, state: &mut MemoryViewState) {
	let width = 16;

	ui.vertical(|ui| {
		ui.vertical(|ui| {
			ui.set_min_width(140.0);
			let mem = cpu.memory.borrow();
			ui.monospace(format!("IR:{:08b}", mem[0xFF0F]));
			ui.monospace(format!("IE:{:08b}", mem[0xFFFF]));

			match state.selected {
				Some(index) => {
					let value = cpu.memory.borrow()[index];
					ui.monospace(format!("INT : {:}", value));
					ui.monospace(format!("BIN : {:08b}", value));
					ui.monospace(format!("HEX : {:02X}", value));
				}
				None => {}
			};
		});
		ui.vertical_centered(|ui| {
			let table = TableBuilder::new(ui)
				.scroll(true)
				.striped(true)
				.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
				.column(Size::Absolute {
					initial: (40.0),
					range: (10.0, 40.0),
				})
				.columns(
					Size::Absolute {
						initial: (15.0),
						range: (10.0, 20.0),
					},
					16,
				)
				.header(16.0, |mut header| {
					header.col(|ui| {
						ui.monospace("");
					});
					for i in 0..16 {
						header.col(|ui| {
							ui.monospace(format!("{:02X}", i * width));
						});
					}
				});

			table.body(|body| {
				body.rows(16.0, 0x10000 / width, |row_index, mut row| {
					row.col(|ui| {
						ui.monospace(format!("{:04X}", row_index * width));
					});

					let pc = cpu.registers.pc;

					for i in 0..width {
						let index: u16 = (row_index * width + i).try_into().unwrap_or(0);
						row.col(|ui| {
							let text = RichText::new(format!("{:02X}", cpu.memory.borrow()[index]))
								.monospace()
								.color(match index {
									p if p == pc => Rgba::RED,
									_ => Rgba::WHITE,
								})
								.background_color(match (state.selected, state.hovering) {
									(Some(i), _) if i == index => {
										Rgba::from(Color32::from_rgb(80, 80, 80))
									}
									(_, Some(i)) if i == index => {
										Rgba::from(Color32::from_rgb(60, 60, 60))
									}
									(_, _) => Rgba::TRANSPARENT,
								});
							let label = Label::new(text).sense(Sense::click());

							let instance = ui.add(label);

							if instance.hovered() {
								_ = state.hovering.insert(index);
							}

							if instance.clicked() {
								_ = state.selected.insert(index);
							}
						});
					}
				});
			});
		});
	});
}
