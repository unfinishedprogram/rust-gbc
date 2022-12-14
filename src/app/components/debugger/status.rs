use crate::emulator::{
	self, memory_mapper::MemoryMapper, state::EmulatorState, timer_controller::TimerController,
};
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

			ui.monospace(format!("INT ENABLE GLOBAL:{}", cpu.interrupt_enable));
			ui.monospace(format!("IE:{:<08b}", emulator.read(0xFFFF)));
			ui.monospace(format!("IR:{:<08b}", emulator.read(0xFF0F)));
			ui.monospace(format!("Timer Enabled:{}", emulator.is_enabled()));
			ui.monospace(format!("Input:{:<08b}", emulator.read(0xFF00)));
			ui.monospace(format!("RawIn:{:<08b}", emulator.raw_joyp_input));
		});
	});
}
