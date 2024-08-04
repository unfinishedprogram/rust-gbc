use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::util::bits::*;

use super::{
	renderer::{AddressingMode, SpriteHeight},
	FetcherMode,
};

bitflags! {
	#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
	struct Flags: u8 {
		const BG_DISPLAY_ENABLE = BIT_0;
		const OBJ_DISPLAY_ENABLE = BIT_1;
		const OBJ_SIZE = BIT_2;
		const BG_TILE_MAP_DISPLAY_SELECT = BIT_3;
		const BG_AND_WINDOW_TILE_DATA_SELECT = BIT_4;
		const WINDOW_DISPLAY_ENABLE = BIT_5;
		const WINDOW_TILE_MAP_DISPLAY_SELECT = BIT_6;
		const DISPLAY_ENABLE = BIT_7;

		const WN_BG_ENABLED = Flags::WINDOW_DISPLAY_ENABLE.bits() | Flags::BG_DISPLAY_ENABLE.bits();
	}
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Lcdc {
	flags: Flags,
}

impl Lcdc {
	pub fn read(&self) -> u8 {
		self.flags.bits()
	}

	pub fn write(&mut self, value: u8) {
		self.flags = Flags::from_bits_truncate(value)
	}

	pub fn obj_size(&self) -> SpriteHeight {
		if self.flags.contains(Flags::OBJ_SIZE) {
			SpriteHeight::Double
		} else {
			SpriteHeight::Single
		}
	}

	pub fn obj_enable(&self) -> bool {
		self.flags.contains(Flags::OBJ_DISPLAY_ENABLE)
	}

	pub fn addressing_mode(&self) -> AddressingMode {
		if self.flags.contains(Flags::BG_AND_WINDOW_TILE_DATA_SELECT) {
			AddressingMode::Signed
		} else {
			AddressingMode::Unsigned
		}
	}

	pub fn tile_map_offset(&self, mode: FetcherMode) -> u16 {
		let flag = match mode {
			FetcherMode::Window => Flags::WINDOW_TILE_MAP_DISPLAY_SELECT,
			FetcherMode::Background => Flags::BG_TILE_MAP_DISPLAY_SELECT,
		};

		if self.flags.contains(flag) {
			0x1C00
		} else {
			0x1800
		}
	}

	pub fn win_enabled(&self) -> bool {
		self.flags.contains(Flags::WN_BG_ENABLED)
	}

	pub fn bg_enabled(&self) -> bool {
		self.flags.contains(Flags::BG_DISPLAY_ENABLE)
	}

	pub fn display_enabled(&self) -> bool {
		self.flags.contains(Flags::DISPLAY_ENABLE)
	}
}
