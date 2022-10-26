use crate::emulator::{self, state::EmulatorState};
use egui::Ui;
use emulator::cpu::registers::{CPURegister16::*, CPURegister8::*};

pub fn draw_cpu_status(ui: &mut Ui, emulator: &EmulatorState) {
	let cpu = &emulator.cpu_state;
	// let ppu = &emulator.ppu_state;

	ui.heading("CPU Info");
	ui.set_width(ui.available_width());
	ui.separator();

	ui.horizontal(|ui| {
		ui.vertical(|ui| {
			ui.monospace(format!("A :{:02X}", cpu.registers[A]));
			ui.monospace(format!("B :{:02X}", cpu.registers[B]));
			ui.monospace(format!("C :{:02X}", cpu.registers[C]));
			ui.monospace(format!("D :{:02X}", cpu.registers[D]));
			ui.monospace(format!("E :{:02X}", cpu.registers[E]));
			ui.monospace(format!("F :{:02X}", cpu.registers[F]));
			ui.monospace(format!("H :{:02X}", cpu.registers[H]));
			ui.monospace(format!("L :{:02X}", cpu.registers[L]));
		});

		ui.vertical_centered(|ui| {
			ui.monospace(format!("IE:{}", cpu.interrupt_enable));
			ui.monospace(format!("PC:{:04X}", cpu.registers.get_u16(PC)));
			ui.monospace(format!("SP:{:04X}", cpu.registers.get_u16(SP)));
			ui.monospace(format!("AF:{:04X}", cpu.registers.get_u16(AF)));
			ui.monospace(format!("BC:{:04X}", cpu.registers.get_u16(BC)));
			ui.monospace(format!("DE:{:04X}", cpu.registers.get_u16(DE)));
			ui.monospace(format!("HL:{:04X}", cpu.registers.get_u16(HL)));
			ui.monospace(format!("SP:{:04X}", cpu.registers.get_u16(SP)));
			ui.monospace(format!("PC:{:04X}", cpu.registers.get_u16(PC)));
		});
	});

	ui.separator();
	ui.horizontal(|ui| {
		ui.vertical(|ui| {
			// ui.monospace(format!("Z :{}", cpu.get_flag(Flag::Z)));
			// ui.monospace(format!("N :{}", cpu.get_flag(Flag::N)));
			// ui.monospace(format!("H :{}", cpu.get_flag(Flag::H)));
			// ui.monospace(format!("C :{}", cpu.get_flag(Flag::C)));
		});
		ui.vertical(|ui| {
			ui.heading("PPU Info");
			// ui.monospace(format!("Mode: {:?}", ppu.get_mode()));
			// ui.monospace(format!("LY:   {:?}", ppu.get_ly()));
		});
	});
}
