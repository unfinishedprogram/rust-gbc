use super::breakpoint_manager;
use crate::emulator::Emulator;
use crate::memory::Memory;
use crate::{app::drawable::DrawableMut, cpu::Cpu};
use breakpoint_manager::BreakpointManager;
use egui::{Align, Color32, Label, Layout, Rgba, RichText, ScrollArea, Sense, Style, Ui};
use egui_extras::{Size, TableBuilder};
use std::collections::HashSet;
use std::{cell::RefCell, rc::Rc};

pub struct MemoryView {
	selected: Option<u16>,
	hovering: Option<u16>,
	focus_cell: Option<usize>,
	scroll_offset: f32,
}

impl Default for MemoryView {
	fn default() -> Self {
		Self {
			selected: None,
			hovering: None,
			scroll_offset: 0.0,
			focus_cell: None,
		}
	}
}

impl MemoryView {
	pub fn focus_cell(&mut self, cell: usize) {
		self.focus_cell.insert(cell);
	}
	pub fn clear_focus(&mut self) {
		self.focus_cell = None;
	}
}

impl MemoryView {
	pub fn draw(
		&mut self,
		ui: &mut Ui,
		emulator: &mut Emulator,
		breakpoint_manager: &mut BreakpointManager,
	) {
		ui.vertical(|ui| {
			ui.vertical_centered(|ui| {
				let available_rect = ui.available_rect_before_wrap();
				let cell_height = 18.0;
				let cell_count: usize = f32::ceil(available_rect.height() / cell_height) as usize;

				if let Some(cell) = self.focus_cell {
					self.scroll_offset = (usize::saturating_sub(cell, 5) as f32) * cell_height;
				}

				let min_cell = (self.scroll_offset / cell_height) as usize;
				let max_cell = min_cell + cell_count;

				let total_height = (0x10000) as f32 * cell_height;

				let layout_start = cell_height * min_cell as f32;
				let layout_end = cell_height * max_cell as f32;

				ui.style_mut().spacing.button_padding = (2.0, 2.0).into();

				ScrollArea::vertical()
					.vertical_scroll_offset(layout_start)
					.show(ui, |ui| {
						ui.add_space(layout_start);
						ui.visuals_mut().widgets.noninteractive.bg_fill = Color32::BLACK;

						for i in min_cell..max_cell {
							let color = if let Some(selected) = self.focus_cell {
								if selected == i {
									Color32::GREEN
								} else {
									Color32::WHITE
								}
							} else {
								Color32::WHITE
							};

							ui.horizontal(|ui| {
								let break_state = breakpoint_manager.break_on(i as u16);
								if ui.button(if break_state { "🌑" } else { "⭕" }).clicked() {
									if break_state {
										breakpoint_manager.remove_breakpoint(i as u16);
									} else {
										breakpoint_manager.add_breakpoint(i as u16);
									}
								}
								ui.colored_label(color, format!("{:04X}", i));
							});
						}

						ui.add_space(total_height - layout_end);
					});
			});
		});
	}
}
