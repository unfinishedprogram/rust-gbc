use egui::Ui;
use gameboy::Gameboy;
use sm83::{
	flags::{cpu, Flags},
	registers::Addressable,
};

use crate::bool;

pub fn show_cpu_info(gb: &Gameboy, ui: &mut Ui) {
	let cpu = &gb.cpu_state;

	use sm83::registers::{CPURegister16::*, CPURegister8::*};

	ui.horizontal(|ui| {
		ui.vertical(|ui| {
			ui.label(format!("A:{:02x}", cpu.read(A)));
			ui.label(format!("B:{:02x}", cpu.read(B)));
			ui.label(format!("C:{:02x}", cpu.read(C)));
			ui.label(format!("D:{:02x}", cpu.read(D)));
			ui.label(format!("E:{:02x}", cpu.read(E)));
			ui.label(format!("F:{:02x}", cpu.read(F)));
			ui.label(format!("H:{:02x}", cpu.read(H)));
			ui.label(format!("L:{:02x}", cpu.read(L)));
		});

		ui.separator();

		ui.vertical(|ui| {
			ui.label(format!("PC:{:04x}", cpu.read(PC)));
			ui.label(format!("SP:{:04x}", cpu.read(SP)));

			ui.separator();

			ui.label(format!("AF:{:04x}", cpu.read(AF)));
			ui.label(format!("BC:{:04x}", cpu.read(BC)));
			ui.label(format!("DE:{:04x}", cpu.read(DE)));
			ui.label(format!("HL:{:04x}", cpu.read(HL)));

			ui.horizontal(|ui| {
				ui.label(bool!("Z:{}", cpu.get_flag(cpu::Z)));
				ui.label(bool!("N:{}", cpu.get_flag(cpu::N)));
				ui.label(bool!("H:{}", cpu.get_flag(cpu::H)));
				ui.label(bool!("C:{}", cpu.get_flag(cpu::C)));
			})
		});
	});
	ui.separator();
	ui.label(bool!("Halted:{}", cpu.halted));
	ui.label(bool!("IME:{}", cpu.ime()));
	ui.label(format!("IE:{:08b}", cpu.interrupt_enable));
	ui.label(format!("IR:{:08b}", cpu.interrupt_request));
	ui.label(bool!("Booting:{}", gb.booting));
}
