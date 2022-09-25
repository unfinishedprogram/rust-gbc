use crate::ppu::Ppu;
use egui::Context;

pub fn ppu_view(ctx: &Context, ppu: &Ppu) {
	egui::Window::new("PPU State")
		.resizable(true)
		.show(ctx, |ui| {
			ui.monospace(format!("Mode: {:?}", ppu.get_mode()));
			ui.monospace(format!("LY:   {:?}", ppu.get_ly()));
		});
}
