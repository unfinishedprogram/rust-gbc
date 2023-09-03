use serde::{Deserialize, Serialize};

use crate::util::bits::*;
use std::{cmp::PartialOrd, option::Option};

use super::tile_data::TileAttributes;
#[derive(Clone, Serialize, Deserialize)]
pub struct Sprite {
	pub addr: u16,
	pub x: u8,
	pub y: u8,
	pub tile_attributes: TileAttributes,
	pub tile_index: u8,
	pub pallet_address: bool,
}

impl Sprite {
	pub fn new(addr: u16, bytes: [u8; 4]) -> Self {
		let [y, x, tile_index, attributes] = bytes;
		let pallet_address = attributes & BIT_4 == BIT_4;

		// bg_priority, v-flip and h-flip are inverted for sprites
		let tile_attributes = TileAttributes::new(attributes ^ 0b11100000);
		Self {
			addr,
			x,
			y,
			tile_attributes,
			tile_index,
			pallet_address,
		}
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

impl Eq for Sprite {}

impl PartialOrd for Sprite {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}
