use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::util::bits::*;

bitflags! {
	#[derive(Default, Serialize, Deserialize)]
	pub struct LCDC: u8 {
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
