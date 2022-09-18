use crate::cpu::flags::{Flag, Flags};
use crate::cpu::registers::CPURegister16::*;
use crate::cpu::registers::CPURegister8::*;
use crate::cpu::Cpu;
use egui::Context;

pub fn state_view(ctx: &Context, cpu: &Cpu) {
	egui::Window::new("Registers")
		.resizable(true)
		.vscroll(true)
		.show(ctx, |ui| {
			ui.monospace(format!("PC:{:x}", cpu.registers.get_u16(PC)));
			ui.monospace(format!("SP:{:x}", cpu.registers.get_u16(SP)));
			ui.separator();
			ui.monospace(format!("A:{:x}", cpu.registers[A]));
			ui.monospace(format!("B:{:x}", cpu.registers[B]));
			ui.monospace(format!("C:{:x}", cpu.registers[C]));
			ui.monospace(format!("D:{:x}", cpu.registers[D]));
			ui.monospace(format!("E:{:x}", cpu.registers[E]));
			ui.monospace(format!("F:{:x}", cpu.registers[F]));
			ui.monospace(format!("H:{:x}", cpu.registers[H]));
			ui.monospace(format!("L:{:x}", cpu.registers[L]));
			ui.separator();
			ui.monospace(format!("PC:{:x}", cpu.registers.get_u16(PC)));
			ui.monospace(format!("SP:{:x}", cpu.registers.get_u16(SP)));
			ui.monospace(format!("AF:{:x}", cpu.registers.get_u16(AF)));
			ui.monospace(format!("BC:{:x}", cpu.registers.get_u16(BC)));
			ui.monospace(format!("DE:{:x}", cpu.registers.get_u16(DE)));
			ui.monospace(format!("HL:{:x}", cpu.registers.get_u16(HL)));
			ui.separator();
			ui.monospace(format!("Z:{}", cpu.get_flag(Flag::Z)));
			ui.monospace(format!("N:{}", cpu.get_flag(Flag::N)));
			ui.monospace(format!("H:{}", cpu.get_flag(Flag::H)));
			ui.monospace(format!("C:{}", cpu.get_flag(Flag::C)));
		});
}
