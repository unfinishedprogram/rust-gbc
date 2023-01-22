use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::util::bits::*;

// Memory addresses of flag registers
pub const INTERRUPT_ENABLE: u16 = 0xFFFF;
pub const INTERRUPT_REQUEST: u16 = 0xFF0F;
pub const JOY_PAD: u16 = 0xFF00;
pub const LCDC: u16 = 0xFF40;
pub const STAT: u16 = 0xFF41;
pub const TIMER: u16 = 0xFF07;

// Interrupt flag masks, Same for INTERRUPT_ENABLE andINTERRUPT_REQUEST
pub const INT_V_BLANK: u8 = BIT_0;
pub const INT_LCD_STAT: u8 = BIT_1;
pub const INT_TIMER: u8 = BIT_2;
pub const INT_SERIAL: u8 = BIT_3;
pub const INT_JOY_PAD: u8 = BIT_4;

// Stat Flag
pub const STAT_LYC_EQ_LY: u8 = BIT_2;
pub const STAT_H_BLANK_IE: u8 = BIT_3;
pub const STAT_V_BLANK_IE: u8 = BIT_4;
pub const STAT_OAM_IE: u8 = BIT_5;
pub const STAT_LYC_EQ_LY_IE: u8 = BIT_6;

bitflags! {
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

// JoyPad Flags
pub const JOYP_RIGHT_OR_A: u8 = BIT_0;
pub const JOYP_LEFT_OR_B: u8 = BIT_1;
pub const JOYP_UP_OR_SELECT: u8 = BIT_2;
pub const JOYP_DOWN_OR_START: u8 = BIT_3;
pub const JOYP_SELECT_DIRECTION_BUTTONS: u8 = BIT_4;
pub const JOYP_SELECT_ACTION_BUTTONS: u8 = BIT_5;
