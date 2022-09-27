use crate::{
	cpu::{
		flags::{Flag, Flags},
		registers::{CPURegister16::*, CPURegister8::*},
	},
	emulator::Emulator,
};
use egui::Ui;

pub fn status_view(ui: &mut Ui, emulator: &Emulator) {
	let cpu = &emulator.cpu;
	let ppu = &emulator.ppu;
	ui.vertical(|ui| {
		ui.set_max_width(70.0);
		ui.heading("CPU Info");

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

		ui.heading("PPU Info");

		ui.monospace(format!("Mode: {:?}", ppu.get_mode()));
		ui.monospace(format!("LY:   {:?}", ppu.get_ly()));
	});
}
