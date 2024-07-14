use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::util::bits::*;

use super::PPUMode;

bitflags! {
	#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
	#[serde(transparent)]
	pub struct Stat:u8 {
		const LYC_EQ_LY = BIT_2;
		const H_BLANK_IE = BIT_3;
		const V_BLANK_IE = BIT_4;
		const OAM_IE = BIT_5;
		const LYC_EQ_LY_IE = BIT_6;
		const UNUSED = BIT_7;
	}
}

impl Stat {
	pub fn read(&self, enabled: bool) -> u8 {
		if enabled {
			self.bits() | Self::UNUSED.bits()
		} else {
			Self::UNUSED.bits()
		}
	}

	pub fn int_enable(&self, mode: PPUMode) -> bool {
		match mode {
			PPUMode::Draw => false,
			PPUMode::HBlank => self.contains(Self::H_BLANK_IE),
			PPUMode::VBlank => self.contains(Self::V_BLANK_IE),
			PPUMode::OamScan => self.contains(Self::OAM_IE),
		}
	}

	pub fn lyc_eq_ly(&self) -> bool {
		self.contains(Self::LYC_EQ_LY)
	}

	pub fn lyc_eq_ly_ie(&self) -> bool {
		self.contains(Self::LYC_EQ_LY_IE)
	}

	// Returns true if an interrupt should be triggered
	pub fn set_lyc_eq_ly(&mut self, value: bool) {
		self.set(Self::LYC_EQ_LY, value);
	}
}
