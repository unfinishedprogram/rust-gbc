use serde::{Deserialize, Serialize};

use crate::util::bits::BIT_4;

use super::PPU;

#[derive(Clone, Serialize, Deserialize)]
pub struct Pixel {
	/// a value between 0 and 3
	color: u8,
	/// on CGB a value between 0 and 7 and on DMG this only applies to sprites
	palette: u8,
	// on CGB this is the OAM index for the sprite and on DMG this doesn't exist
	sprite_priority: usize,
	// holds the value of the OBJ-to-BG Priority bit
	background_priority: bool,
}

pub trait PixelFIFO {
	/// Fetches a row of 8 background or window pixels and queues them up to be mixed with sprite pixels
	fn fetch_row(&mut self);
	fn step_fifo(&mut self);
	fn render_pixel(&mut self, x: u8, y: u8, pixel: Pixel);
}

impl PixelFIFO for PPU {
	fn step_fifo(&mut self) {
		if self.fifo_bg.len() <= 8 {
			self.fetch_row();
		}

		if let Some(pixel) = self.fifo_bg.pop_back() {
			self.render_pixel(self.fifo_pixel, self.ly, pixel);
			self.fifo_pixel += 1;
		}
	}

	fn render_pixel(&mut self, x: u8, y: u8, pixel: Pixel) {}

	fn fetch_row(&mut self) {
		let indexing_mode = self.lcdc & BIT_4 != 0;

		// When LCDC.3 is enabled and the X coordinate of the current scanline is not inside the window then tilemap $9C00 is used.
		// When LCDC.6 is enabled and the X coordinate of the current scanline is inside the window then tilemap $9C00 is used.

		// self.cycle += ;
	}
}

// Get tile
// Get tile data low
// Get tile data high
// Sleep
// Push
