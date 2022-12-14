use serde::{Deserialize, Serialize};

use crate::util::bits::*;
use std::{cmp::PartialOrd, option::Option};
#[derive(Clone, Eq, Serialize, Deserialize)]
pub struct Sprite {
	pub x: u8,
	pub y: u8,
	pub flip_y: bool,
	pub flip_x: bool,
	pub above_bg: bool,
	pub tile_index: u8,
	pub pallet_address: u16,
	pub addr: u16,
}

impl Sprite {
	pub fn new(addr: u16, bytes: (u8, u8, u8, u8)) -> Self {
		let (y, x, tile_index, attributes) = bytes;
		let above_bg = attributes & BIT_7 == 0;
		let flip_y = attributes & BIT_6 != 0;
		let flip_x = attributes & BIT_5 != 0;
		let pallet_address = if attributes & BIT_4 != 0 {
			0xFF49
		} else {
			0xFF48
		};

		Self {
			addr,
			x,
			y,
			flip_y,
			flip_x,
			above_bg,
			tile_index,
			pallet_address,
		}
	}

	pub fn is_visible(&self) -> bool {
		self.x > 0 && self.y > 0 && self.x <= 168 && self.y <= 160
	}
}

impl PartialEq for Sprite {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.addr == other.addr
	}
}

impl Ord for Sprite {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match other.x.cmp(&self.x) {
			std::cmp::Ordering::Equal => other.addr.cmp(&self.addr),
			o => o,
		}
	}
}

impl PartialOrd for Sprite {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}
