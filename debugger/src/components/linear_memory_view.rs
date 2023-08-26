use egui::style::Spacing;
use egui::{Align, Rgba, Style, Ui, Vec2};
use egui_extras::{Column, TableBuilder};
use gameboy::Gameboy;
use sm83::memory_mapper::MemoryMapper;
use sm83::{Instruction, SM83};

use crate::memory_map::get_addr_info;

type CompiledEntry = (u16, Instruction, String);

#[derive(Default)]
pub struct LinearMemoryView {
	instructions: Option<Vec<CompiledEntry>>,
	keep_pc_in_view: bool,
}

pub fn generate_instructions(gb: &Gameboy) -> Vec<CompiledEntry> {
	let mut gb = gb.clone();

	let mut instructions = vec![];

	gb.cpu_state.registers.pc = 0;
	while gb.cpu_state.registers.pc < 0xFFFF {
		let pc = gb.cpu_state.registers.pc;
		let instruction = gb.fetch_next_instruction();
		let mut bytes = vec![];
		let new_pc = gb.cpu_state.registers.pc;

		for addr in pc..new_pc {
			bytes.push(format!("{:02X}", gb.read(addr)));
		}
		let bytes = bytes.join(", ");

		instructions.push((pc, instruction, bytes));
	}

	instructions
}

impl LinearMemoryView {
	pub fn draw(&mut self, gameboy: &Gameboy, ui: &mut Ui) {
		let pc = gameboy.cpu_state.registers.pc;

		let instructions = self
			.instructions
			.get_or_insert_with(|| generate_instructions(gameboy));

		ui.set_min_height(260.0);

		ui.horizontal(|ui| {
			if ui.button("Decompile").clicked() {
				*instructions = generate_instructions(gameboy);
			}

			ui.checkbox(&mut self.keep_pc_in_view, "Lock View")
		});

		ui.set_style(Style {
			spacing: Spacing {
				item_spacing: Vec2 { x: 0.0, y: 0.0 },
				..Default::default()
			},
			..Default::default()
		});

		ui.separator();

		let mut row = 0;
		for (index, (addr, _, _)) in instructions.iter().enumerate() {
			if addr >= &pc {
				row = index;
				break;
			}
		}

		if self.keep_pc_in_view {
			TableBuilder::new(ui).scroll_to_row(row, Some(Align::Center))
		} else {
			TableBuilder::new(ui)
		}
		.striped(true)
		.resizable(true)
		.column(Column::exact(40.0))
		.column(Column::remainder())
		.column(Column::remainder())
		.column(Column::remainder())
		.vscroll(true)
		.body(|body| {
			body.rows(20.0, instructions.len(), |index, mut row| {
				let (addr, inst, bytes) = &instructions[index];
				let color = if pc == *addr { Rgba::RED } else { Rgba::WHITE };

				row.col(|ui| {
					ui.colored_label(color, format!("{addr:04X}"));
				});

				row.col(|ui| {
					ui.colored_label(color, format!("{:?}", inst));
				});

				row.col(|ui| {
					ui.colored_label(color, bytes);
				});

				row.col(|ui| {
					ui.colored_label(color, format!("{:?}", get_addr_info(*addr).0));
				});
			});
		});
	}
}
