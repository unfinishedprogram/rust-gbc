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
			ui.monospace(format!("IE:{}", cpu.interrupt_enable));
			ui.monospace(format!("PC:{:04X}", cpu.registers.get_u16(PC)));
			ui.monospace(format!("SP:{:04X}", cpu.registers.get_u16(SP)));
			ui.separator();
			ui.monospace(format!("A :{:02X}", cpu.registers[A]));
			ui.monospace(format!("B :{:02X}", cpu.registers[B]));
			ui.monospace(format!("C :{:02X}", cpu.registers[C]));
			ui.monospace(format!("D :{:02X}", cpu.registers[D]));
			ui.monospace(format!("E :{:02X}", cpu.registers[E]));
			ui.monospace(format!("F :{:02X}", cpu.registers[F]));
			ui.monospace(format!("H :{:02X}", cpu.registers[H]));
			ui.monospace(format!("L :{:02X}", cpu.registers[L]));
			ui.separator();
			ui.monospace(format!("AF:{:04X}", cpu.registers.get_u16(AF)));
			ui.monospace(format!("BC:{:04X}", cpu.registers.get_u16(BC)));
			ui.monospace(format!("DE:{:04X}", cpu.registers.get_u16(DE)));
			ui.monospace(format!("HL:{:04X}", cpu.registers.get_u16(HL)));
			ui.monospace(format!("SP:{:04X}", cpu.registers.get_u16(SP)));
			ui.monospace(format!("PC:{:04X}", cpu.registers.get_u16(PC)));
			ui.separator();
			ui.monospace(format!("Z :{}", cpu.get_flag(Flag::Z)));
			ui.monospace(format!("N :{}", cpu.get_flag(Flag::N)));
			ui.monospace(format!("H :{}", cpu.get_flag(Flag::H)));
			ui.monospace(format!("C :{}", cpu.get_flag(Flag::C)));
		});
}
