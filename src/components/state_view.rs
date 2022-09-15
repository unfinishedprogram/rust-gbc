use crate::cpu::flags::{Flag, Flags};
use crate::cpu::values::get_as_u16;
use crate::cpu::Cpu;
use egui::Context;

pub fn state_view(ctx: &Context, cpu: &Cpu) {
	egui::Window::new("Registers")
		.resizable(true)
		.vscroll(true)
		.show(ctx, |ui| {
			ui.monospace(format!("PC:{:x}", cpu.registers.pc));
			ui.monospace(format!("SP:{:x}", cpu.registers.sp));
			ui.separator();
			ui.monospace(format!("A:{:x}", cpu.registers.bytes[0]));
			ui.monospace(format!("B:{:x}", cpu.registers.bytes[1]));
			ui.monospace(format!("C:{:x}", cpu.registers.bytes[2]));
			ui.monospace(format!("D:{:x}", cpu.registers.bytes[3]));
			ui.monospace(format!("E:{:x}", cpu.registers.bytes[4]));
			ui.monospace(format!("F:{:x}", cpu.registers.bytes[5]));
			ui.monospace(format!("H:{:x}", cpu.registers.bytes[6]));
			ui.monospace(format!("L:{:x}", cpu.registers.bytes[7]));
			ui.separator();
			ui.monospace(format!("PC:{:x}", cpu.registers.pc));
			ui.monospace(format!("SP:{:x}", cpu.registers.sp));
			ui.monospace(format!(
				"AF:{:x}",
				get_as_u16(&cpu.registers.bytes[0], &cpu.registers.bytes[5])
			));
			ui.monospace(format!(
				"BC:{:x}",
				get_as_u16(&cpu.registers.bytes[1], &cpu.registers.bytes[2])
			));
			ui.monospace(format!(
				"DE:{:x}",
				get_as_u16(&cpu.registers.bytes[3], &cpu.registers.bytes[4])
			));
			ui.monospace(format!(
				"HL:{:x}",
				get_as_u16(&cpu.registers.bytes[6], &cpu.registers.bytes[7])
			));

			ui.separator();
			ui.monospace(format!("Z:{}", cpu.get_flag(Flag::Z)));
			ui.monospace(format!("N:{}", cpu.get_flag(Flag::N)));
			ui.monospace(format!("H:{}", cpu.get_flag(Flag::H)));
			ui.monospace(format!("C:{}", cpu.get_flag(Flag::C)));
		});
}
