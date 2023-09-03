use serde::{Deserialize, Serialize};

use super::VRAMBank;
use crate::util::bits::*;

pub struct TileData(pub u16, pub Option<TileAttributes>);

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct TileAttributes {
	byte: u8,
}

impl TileAttributes {
	#[inline]
	pub fn new(attributes: u8) -> TileAttributes {
		TileAttributes { byte: attributes }
	}

	#[inline]
	pub fn bg_priority(self) -> bool {
		self.byte & BIT_7 == BIT_7
	}

	#[inline]
	pub fn vertical_flip(self) -> bool {
		self.byte & BIT_6 == BIT_6
	}

	#[inline]
	pub fn horizontal_flip(self) -> bool {
		self.byte & BIT_5 == BIT_5
	}

	#[inline]
	pub fn v_ram_bank(self) -> VRAMBank {
		VRAMBank::from((self.byte >> 3) & 1)
	}

	#[inline]
	pub fn palette_number(self) -> u8 {
		self.byte & 0b111
	}
}
