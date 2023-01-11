use crate::Gameboy;

struct Pixel {
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
	fn fetch_pixels(&mut self);
}

impl PixelFIFO for Gameboy {
	fn fetch_pixels(&mut self) {}
}

// Get tile
// Get tile data low
// Get tile data high
// Sleep
// Push
