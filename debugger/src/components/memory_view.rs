use std::str::from_utf8;

use egui::style::Spacing;
use egui::{RichText, Style, Ui, Vec2};
use egui_extras::{Column, TableBuilder};
use gameboy::Gameboy;
use sm83::memory_mapper::MemoryMapper;

#[derive(Default)]
pub struct MemoryView {
	selected: Option<u16>,
}

impl MemoryView {
	pub fn draw(&mut self, gameboy: &Gameboy, ui: &mut Ui) {
		ui.horizontal(|ui| {
			ui.set_min_height(260.0);
			ui.set_style(Style {
				spacing: Spacing {
					item_spacing: Vec2 { x: 0.0, y: 0.0 },
					..Default::default()
				},
				..Default::default()
			});
			ui.vertical(|ui| {
				let addr = self.selected.unwrap_or_default();
				let value = gameboy.read(addr);
				ui.monospace(format!("Addr   :{:04X}", addr));
				ui.monospace(format!("Base16 :{value:02X}"));
				ui.monospace(format!("Base10 :{value:}"));
				ui.monospace(format!("Binary :{value:08b}"));
				ui.monospace(format!("Char   :{:}", from_utf8(&[value]).unwrap_or("Err")));
			});
			ui.separator();
			ui.vertical(|ui| {
				TableBuilder::new(ui)
					.striped(true)
					.column(Column::exact(40.0))
					.columns(Column::exact(26.0), 16)
					.vscroll(true)
					.header(22.0, |mut ui| {
						ui.col(|ui| {
							ui.monospace(" ");
						});

						for i in 0..0x10 {
							ui.col(|ui| {
								ui.monospace(format!("{:0X}", i));
							});
						}
					})
					.body(|body| {
						body.rows(20.0, 0x10000 / 0x10, |index, mut row| {
							row.col(|ui| {
								ui.monospace(format!("{:04X}", index * 16));
							});

							for i in 0..0x10 {
								row.col(|ui| {
									let addr = (index * 16 + i) as u16;
									let text = format!("{:02X}", gameboy.read(addr));
									ui.selectable_value(
										&mut self.selected,
										Some(addr),
										RichText::monospace(text.into()),
									);
								});
							}
						});
					});
			});
		});
	}
}
