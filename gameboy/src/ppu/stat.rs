use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::util::bits::*;

bitflags! {
	#[derive(Serialize, Deserialize, Default)]
	pub struct STAT:u8 {
		const PPU_MODE = BIT_0 | BIT_1;
		const LYC_EQ_LY = BIT_2;
		const H_BLANK_IE = BIT_3;
		const V_BLANK_IE = BIT_4;
		const OAM_IE = BIT_5;
		const LYC_EQ_LY_IE = BIT_6;

		const READ_ONLY = Self::PPU_MODE.bits | Self::LYC_EQ_LY.bits;
	}
}
