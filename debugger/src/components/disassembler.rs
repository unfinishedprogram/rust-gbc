use std::collections::HashMap;

use egui::style::Spacing;
use egui::{Align, Button, Color32, Rgba, Stroke, Style, Ui, Vec2};
use egui_extras::{Column, TableBuilder};
use gameboy::Gameboy;
use sm83::instruction::Fetch;
use sm83::memory_mapper::MemoryMapper;
use sm83::registers::{Addressable, CPURegister16};
use sm83::Instruction;

use crate::memory_map::get_addr_info;

pub struct DisassembledInstruction {
	addr: u16,
	instruction: Instruction,
	bytes: String,
}

#[derive(Default)]
pub struct Disassembler {
	instructions: Option<Vec<DisassembledInstruction>>,
	keep_pc_in_view: bool,
	breakpoints: HashMap<u16, bool>,
}

pub fn generate_instructions(gb: &Gameboy) -> Vec<DisassembledInstruction> {
	let mut gb = gb.clone();

	let mut instructions = vec![];

	gb.cpu_state.write(CPURegister16::PC, 0);

	loop {
		let pc = gb.cpu_state.read(CPURegister16::PC);
		let instruction = gb.fetch();
		let new_pc = gb.cpu_state.read(CPURegister16::PC);

		if new_pc <= pc {
			// Detect looping back around to the start of memory
			// This is likely to cause an infinite loop if not avoided
			break;
		}

		let mut bytes = vec![];
		for addr in pc..new_pc {
			bytes.push(format!("{:02X}", gb.read(addr)));
		}
		let bytes = bytes.join(", ");

		instructions.push(DisassembledInstruction {
			addr: pc,
			instruction,
			bytes,
		});
	}

	instructions
}

impl Disassembler {
	pub fn draw(&mut self, gameboy: &Gameboy, ui: &mut Ui) {
		let pc = gameboy.cpu_state.read(CPURegister16::PC);

		let instructions = self
			.instructions
			.get_or_insert_with(|| generate_instructions(gameboy));

		ui.horizontal(|ui| {
			if ui.button("Disassemble").clicked() {
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
		for (index, DisassembledInstruction { addr, .. }) in instructions.iter().enumerate() {
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
		.column(Column::exact(25.0))
		.column(Column::exact(40.0))
		.column(Column::remainder())
		.column(Column::remainder())
		.column(Column::remainder())
		.vscroll(true)
		.body(|body| {
			body.rows(20.0, instructions.len(), |mut row| {
				let index = row.index();
				let DisassembledInstruction {
					addr,
					instruction,
					bytes,
				} = &instructions[index];
				let color = if pc == *addr { Rgba::RED } else { Rgba::WHITE };

				row.col(|ui| {
					let is_breakpoint = self.breakpoints.contains_key(addr);
					let text = if is_breakpoint { "🌑" } else { "🌕" };

					if ui
						.add(
							Button::new(text)
								.small()
								.stroke(Stroke::NONE)
								.fill(Color32::TRANSPARENT),
						)
						.clicked()
					{
						if is_breakpoint {
							self.breakpoints.remove(addr)
						} else {
							self.breakpoints.insert(*addr, true)
						};
					}
				});

				row.col(|ui| {
					ui.colored_label(color, format!("{addr:04X}"));
				});

				row.col(|ui| {
					ui.colored_label(color, format!("{instruction:?}"));
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

	pub fn should_break(&self, gameboy: &Gameboy) -> bool {
		let pc = gameboy.cpu_state.read(CPURegister16::PC);
		self.breakpoints.contains_key(&pc)
	}
}
