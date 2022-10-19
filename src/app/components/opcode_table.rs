use crate::emulator::cpu::Cpu;
use egui::Ui;
use egui_extras::{Size, TableBuilder};

pub fn opcode_table(ui: &mut Ui, cpu: &mut Cpu) {
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
					cpu.registers.pc = 0;
					{
						let mut mem = cpu.memory.borrow_mut();
						mem.write(0, ((row_index << 4) + i) as u8);
						mem.write(1, 0);
						mem.write(2, 0);
						mem.write(3, 0);
						mem.write(4, 0);
						mem.write(5, 0);
					}

					row.col(|ui| {
						ui.wrap_text();
						ui.monospace(format!("{:?}", cpu.get_next_instruction()));
					});
				}
			})
		});
}
