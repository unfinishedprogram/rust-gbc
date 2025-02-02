use super::{
	renderer::{AddressingMode, SpriteHeight},
	FetcherMode,
};
use crate::util::bits::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Lcdc(u8);

impl Lcdc {
	const BG_DISPLAY_ENABLE: u8 = BIT_0;
	const OBJ_DISPLAY_ENABLE: u8 = BIT_1;
	const OBJ_SIZE: u8 = BIT_2;
	const BG_TILE_MAP_DISPLAY_SELECT: u8 = BIT_3;
	const BG_AND_WINDOW_TILE_DATA_SELECT: u8 = BIT_4;
	const WINDOW_DISPLAY_ENABLE: u8 = BIT_5;
	const WINDOW_TILE_MAP_DISPLAY_SELECT: u8 = BIT_6;
	const DISPLAY_ENABLE: u8 = BIT_7;

	pub fn read(&self) -> u8 {
		self.0
	}

	pub fn write(&mut self, value: u8) {
		self.0 = value;
	}

	pub fn obj_size(&self) -> SpriteHeight {
		if self.is_set(Self::OBJ_SIZE) {
			SpriteHeight::Double
		} else {
			SpriteHeight::Single
		}
	}

	pub fn obj_enable(&self) -> bool {
		self.is_set(Self::OBJ_DISPLAY_ENABLE)
	}

	pub fn addressing_mode(&self) -> AddressingMode {
		if self.is_set(Self::BG_AND_WINDOW_TILE_DATA_SELECT) {
			AddressingMode::Signed
		} else {
			AddressingMode::Unsigned
		}
	}

	pub fn tile_map_offset(&self, mode: FetcherMode) -> u16 {
		let flag = match mode {
			FetcherMode::Window => Self::WINDOW_TILE_MAP_DISPLAY_SELECT,
			FetcherMode::Background => Self::BG_TILE_MAP_DISPLAY_SELECT,
		};

		if self.is_set(flag) {
			0x1C00
		} else {
			0x1800
		}
	}

	pub fn win_enabled(&self) -> bool {
		self.is_set(Self::WINDOW_DISPLAY_ENABLE | Self::BG_DISPLAY_ENABLE)
	}

	pub fn bg_enabled(&self) -> bool {
		self.is_set(Self::BG_DISPLAY_ENABLE)
	}

	pub fn display_enabled(&self) -> bool {
		self.is_set(Self::DISPLAY_ENABLE)
	}

	fn is_set(&self, bit: u8) -> bool {
		self.0 & bit == bit
	}
}
