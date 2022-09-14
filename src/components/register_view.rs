use crate::cpu::Cpu;
use egui::Context;


pub fn register_view(ctx: &Context, cpu:&Cpu) {
	egui::Window::new("Registers")
		.resizable(true)
		.vscroll(true)
		.show(ctx, |ui| {
			ui.label(format!("PC:{:x}", cpu.registers.pc));
			ui.label(format!("SP:{:x}", cpu.registers.sp));

			ui.label(format!("A:{:x}", cpu.registers.bytes[0]));
			ui.label(format!("B:{:x}", cpu.registers.bytes[1]));
			ui.label(format!("C:{:x}", cpu.registers.bytes[2]));
			ui.label(format!("D:{:x}", cpu.registers.bytes[3]));
			ui.label(format!("E:{:x}", cpu.registers.bytes[4]));
			ui.label(format!("F:{:x}", cpu.registers.bytes[5]));
			ui.label(format!("G:{:x}", cpu.registers.bytes[6]));
			ui.label(format!("H:{:x}", cpu.registers.bytes[7]));
		});
}