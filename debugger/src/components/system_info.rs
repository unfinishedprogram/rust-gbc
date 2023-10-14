use egui::Ui;
use egui_extras::{Column, TableBuilder};
use gameboy::Gameboy;
use sm83::{
	flags::{cpu, Flags},
	registers::Addressable,
	Interrupt, SM83,
};

use crate::bool;

fn interrupt_info(gb: &Gameboy, ui: &mut Ui) {
	ui.label(bool!("IME:{}", gb.cpu_state().ime()));
	ui.label(format!("IE:{:08b}", gb.cpu_state().interrupt_enable));
	ui.label(format!("IR:{:08b}", gb.cpu_state().interrupt_request));
	ui.separator();

	use Interrupt::*;
	let interrupt_labels = [VBlank, LcdStat, Timer, Serial, JoyPad];

	TableBuilder::new(ui)
		.column(Column::exact(80.0))
		.columns(Column::exact(20.0), 2)
		.header(20.0, |mut row| {
			row.col(|ui| {
				ui.label("Int");
			});
			row.col(|ui| {
				ui.label("IE");
			});
			row.col(|ui| {
				ui.label("IR");
			});
		})
		.body(|body| {
			body.rows(20.0, 5, |index, mut row| {
				row.col(|ui| {
					ui.label(format!("{:?}", interrupt_labels[index]));
				});
				row.col(|ui| {
					ui.label(bool!(
						"{}",
						gb.cpu_state().interrupt_enable & interrupt_labels[index] as u8 != 0
					));
				});
				row.col(|ui| {
					ui.label(bool!(
						"{}",
						gb.cpu_state().interrupt_request & interrupt_labels[index] as u8 != 0
					));
				});
			})
		});
}

fn cpu_info(gb: &Gameboy, ui: &mut Ui) {
	let cpu = &gb.cpu_state;
	use sm83::registers::{CPURegister16::*, CPURegister8::*};

	ui.label(bool!("Halted:{}", cpu.halted));
	ui.label(bool!("Booting:{}", gb.booting));

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
}

fn ppu_info(gb: &Gameboy, ui: &mut Ui) {
	let ppu = &gb.ppu;
	let oam = gb.oam_dma.oam_is_accessible();

	ui.monospace(format!("Mode: {:?}", ppu.mode()));
	ui.monospace(format!("LY: {:?}", ppu.get_ly()));
	ui.monospace(format!("LCDC: {:08b}", ppu.read_lcdc()));
	ui.monospace(bool!("Enabled: {}", ppu.is_enabled()));
	ui.monospace(bool!("OAM Dma: {}", !oam));
	ui.monospace(format!("Frame: {}", ppu.frame));
	ui.monospace(format!("Cycle: {}", ppu.cycle));
}

pub fn show_system_info(gb: &Gameboy, ui: &mut Ui) {
	ui.set_min_width(200.0);

	ui.collapsing("CPU Info", |ui| cpu_info(gb, ui));
	ui.collapsing("Interrupts", |ui| interrupt_info(gb, ui));
	ui.collapsing("PPU Info", |ui| ppu_info(gb, ui));
}
