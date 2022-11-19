use crate::emulator::{self, state::EmulatorState};
use egui::Ui;
use emulator::cpu::registers::CPURegister16::*;

pub fn draw_status(ui: &mut Ui, emulator: &EmulatorState) {
	let cpu = &emulator.cpu_state;

	ui.heading("CPU Info");
	ui.set_width(ui.available_width());
	ui.separator();

	ui.horizontal(|ui| {
		ui.vertical(|ui| {
			ui.monospace(format!("PC:{:04X}", cpu.registers.get_u16(PC)));
			ui.monospace(format!("SP:{:04X}", cpu.registers.get_u16(SP)));
			ui.monospace(format!("AF:{:04X}", cpu.registers.get_u16(AF)));
			ui.monospace(format!("BC:{:04X}", cpu.registers.get_u16(BC)));
			ui.monospace(format!("DE:{:04X}", cpu.registers.get_u16(DE)));
			ui.monospace(format!("HL:{:04X}", cpu.registers.get_u16(HL)));
			ui.monospace(format!("SP:{:04X}", cpu.registers.get_u16(SP)));
			ui.monospace(format!("PC:{:04X}", cpu.registers.get_u16(PC)));
		});
		ui.vertical(|_ui| {
			// ui.monospace(format!("Z :{}", cpu.get_flag(Flag::Z)));
			// ui.monospace(format!("N :{}", cpu.get_flag(Flag::N)));
			// ui.monospace(format!("H :{}", cpu.get_flag(Flag::H)));
			// ui.monospace(format!("C :{}", cpu.get_flag(Flag::C)));
		});
	});
	ui.horizontal(|ui| {
		ui.monospace(format!("IE:{}", cpu.interrupt_enable));
	});
}
