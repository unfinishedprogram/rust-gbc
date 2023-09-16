use egui::Ui;
use gameboy::Gameboy;
use sm83::flags::{cpu, Flags};

use crate::bool;

pub fn show_cpu_info(gb: &Gameboy, ui: &mut Ui) {
	let cpu = &gb.cpu_state;
	let r = &cpu.registers;

	use sm83::registers::{CPURegister16::*, CPURegister8::*};

	ui.horizontal(|ui| {
		ui.vertical(|ui| {
			ui.label(format!("A:{:02x}", r[A]));
			ui.label(format!("B:{:02x}", r[B]));
			ui.label(format!("C:{:02x}", r[C]));
			ui.label(format!("D:{:02x}", r[D]));
			ui.label(format!("E:{:02x}", r[E]));
			ui.label(format!("F:{:02x}", r[F]));
			ui.label(format!("H:{:02x}", r[H]));
			ui.label(format!("L:{:02x}", r[L]));
		});

		ui.separator();

		ui.vertical(|ui| {
			ui.label(format!("PC:{:04x}", r[PC]));
			ui.label(format!("SP:{:04x}", r[SP]));

			ui.separator();

			ui.label(format!("AF:{:04x}", r[AF]));
			ui.label(format!("BC:{:04x}", r[BC]));
			ui.label(format!("DE:{:04x}", r[DE]));
			ui.label(format!("HL:{:04x}", r[HL]));

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
	ui.label(bool!("IME:{}", cpu.interrupt_master_enable));
	ui.label(format!("IE:{:08b}", cpu.interrupt_enable));
	ui.label(format!("IR:{:08b}", cpu.interrupt_request));
	ui.label(bool!("Booting:{}", gb.booting));
}
