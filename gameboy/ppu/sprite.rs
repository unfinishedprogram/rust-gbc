use serde::{Deserialize, Serialize};

use crate::util::bits::*;
use std::{cmp::PartialOrd, option::Option};

use super::tile_data::TileAttributes;
#[derive(Clone, Serialize, Deserialize)]
pub struct Sprite {
	pub x: u8,
	pub y: u8,
	pub tile_attributes: TileAttributes,
	pub tile_index: u8,
	pub pallet_address: bool,
	pub addr: u16,
}

impl Sprite {
	pub fn new(addr: u16, bytes: [u8; 4]) -> Self {
		let [y, x, tile_index, attributes] = bytes;

		let above_bg = attributes & BIT_7 == 0;
		let flip_y = attributes & BIT_6 != 0;
		let flip_x = attributes & BIT_5 != 0;
		let pallet_address = attributes & BIT_4 != 0;
		let tile_vram_bank = (attributes >> 3) & 1;
		let palette_number = (attributes & 0b111) as usize;

		Self {
			addr,
			x,
			y,
			tile_attributes: TileAttributes {
				vertical_flip: !flip_y,
				horizontal_flip: !flip_x,
				v_ram_bank: tile_vram_bank as usize,
				bg_priority: above_bg,
				palette_number,
			},
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
impl Eq for Sprite {}

impl PartialOrd for Sprite {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}
