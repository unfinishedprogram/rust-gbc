use egui::Ui;
use gameboy::Gameboy;
use sm83::flags::{cpu::*, Flags};

use crate::bool;

pub fn show_cpu_info(gb: &Gameboy, ui: &mut Ui) {
	let cpu = &gb.cpu_state;
	let r = &cpu.registers;

	use sm83::registers::CPURegister16::*;

	ui.horizontal(|ui| {
		ui.vertical(|ui| {
			ui.label(format!("A:{:02x}", r.bytes[0]));
			ui.label(format!("B:{:02x}", r.bytes[1]));
			ui.label(format!("C:{:02x}", r.bytes[2]));
			ui.label(format!("D:{:02x}", r.bytes[3]));
			ui.label(format!("E:{:02x}", r.bytes[4]));
			ui.label(format!("F:{:02x}", r.bytes[5]));
			ui.label(format!("H:{:02x}", r.bytes[6]));
			ui.label(format!("L:{:02x}", r.bytes[7]));
		});

		ui.separator();

		ui.vertical(|ui| {
			ui.label(format!("PC:{:04x}", r.pc));
			ui.label(format!("SP:{:04x}", r.sp));

			ui.separator();

			ui.label(format!("AF:{:04x}", r.get_u16(AF)));
			ui.label(format!("BC:{:04x}", r.get_u16(BC)));
			ui.label(format!("DE:{:04x}", r.get_u16(DE)));
			ui.label(format!("HL:{:04x}", r.get_u16(HL)));

			ui.horizontal(|ui| {
				ui.label(bool!("Z:{}", cpu.get_flag(Z)));
				ui.label(bool!("N:{}", cpu.get_flag(N)));
				ui.label(bool!("H:{}", cpu.get_flag(H)));
				ui.label(bool!("C:{}", cpu.get_flag(C)));
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
