use egui::Ui;
use egui_extras::{Size, TableBuilder};

use crate::emulator::{cpu::CPU, memory_mapper::MemoryMapper, EmulatorState};

pub fn opcode_table(ui: &mut Ui, cpu: &mut EmulatorState) {
	TableBuilder::new(ui)
		.striped(true)
		.columns(
			Size::Absolute {
				initial: (100.0),
				range: (100.0, 100.0),
			},
			16,
		)
		.body(|body| {
			body.rows(64.0, 16, |row_index, mut row| {
				for i in 0..16 {
					cpu.cpu_state.registers.pc = 0;
					{
						cpu.write(0, ((row_index << 4) + i) as u8);
						cpu.write(1, 0);
						cpu.write(2, 0);
						cpu.write(3, 0);
						cpu.write(4, 0);
						cpu.write(5, 0);
					}

					row.col(|ui| {
						ui.wrap_text();
						ui.monospace(format!("{:?}", cpu.fetch_next_instruction()));
					});
				}
			})
		});
}
