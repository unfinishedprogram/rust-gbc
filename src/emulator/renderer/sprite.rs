use crate::util::bits::*;

pub struct Sprite {
	pub x: u8,
	pub y: u8,
	pub flip_y: bool,
	pub flip_x: bool,
	pub bg_priority: bool,
	pub tile_index: u8,
}

impl Sprite {
	pub fn new(bytes: (u8, u8, u8, u8)) -> Self {
		let (x, y, tile_index, attributes) = bytes;
		if x != 0 && y != 0 {
			log::error!("{bytes:?}");
		}

		let bg_priority = attributes & BIT_7 != 0;
		let flip_y = attributes & BIT_6 != 0;
		let flip_x = attributes & BIT_5 != 0;

		Self {
			x,
			y,
			flip_y,
			flip_x,
			bg_priority,
			tile_index,
		}
	}

	pub fn is_visible(&self) -> bool {
		self.x > 0 && self.y > 0 && self.x <= 168 && self.y <= 160
	}
}
