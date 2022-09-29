use egui::Color32;

pub fn color(val: u32) -> Color32 {
	let [_, r, g, b] = val.to_be_bytes();
	Color32::from_rgb(r as u8, g as u8, b as u8)
}
