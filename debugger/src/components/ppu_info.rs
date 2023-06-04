use egui::Ui;
use gameboy::Gameboy;

use crate::bool;

pub fn show_ppu_info(gb: &Gameboy, ui: &mut Ui) {
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
