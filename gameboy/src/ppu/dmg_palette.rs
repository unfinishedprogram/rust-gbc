use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct DMGPalette([(u8, u8, u8, u8); 4]);

impl Default for DMGPalette {
	fn default() -> Self {
		DMGPalette([
			(0xFF, 0xFF, 0xFF, 0xFF),
			(0xAA, 0xAA, 0xAA, 0xFF),
			(0x55, 0x55, 0x55, 0xFF),
			(0x00, 0x00, 0x00, 0xFF),
		])
	}
}

impl DMGPalette {
	pub fn color_of(&self, color_id: u8) -> (u8, u8, u8, u8) {
		self.0[color_id as usize]
	}
}
