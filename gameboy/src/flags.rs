use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::util::bits::*;

bitflags! {
	#[derive(Serialize, Deserialize, Default)]
	pub struct STATFlags:u8 {
		const READ_ONLY = 0b111;
		const PPU_MODE = BIT_0 | BIT_1;
		const LYC_EQ_LY = BIT_2;
		const H_BLANK_IE = BIT_3;
		const V_BLANK_IE = BIT_4;
		const OAM_IE = BIT_5;
		const LYC_EQ_LY_IE = BIT_6;
	}

	#[derive(Serialize, Deserialize)]
	pub struct LCDFlags: u8 {
		const BG_DISPLAY = BIT_0;
		const OBJ_DISPLAY_ENABLE = BIT_1;
		const OBJ_SIZE = BIT_2;
		const BG_TILE_MAP_DISPLAY_SELECT = BIT_3;
		const BG_AND_WINDOW_TILE_DATA_SELECT = BIT_4;
		const WINDOW_DISPLAY_ENABLE = BIT_5;
		const WINDOW_TILE_MAP_DISPLAY_SELECT = BIT_6;
		const DISPLAY_ENABLE = BIT_7;
	}
}
