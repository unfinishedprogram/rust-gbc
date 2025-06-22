use super::PPUMode;
use crate::util::bits::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct Stat(u8);

impl_bitlike!(Stat);

impl Stat {
	const MODE_MASK: u8 = BIT_0 | BIT_1;
	const LYC_EQ_LY: u8 = BIT_2;
	const H_BLANK_IE: u8 = BIT_3;
	const V_BLANK_IE: u8 = BIT_4;
	const OAM_IE: u8 = BIT_5;
	const LYC_EQ_LY_IE: u8 = BIT_6;
	const UNUSED: u8 = BIT_7;

	pub fn read(&self, enabled: bool) -> u8 {
		if enabled {
			self.0 | Self::UNUSED
		} else {
			Self::UNUSED
		}
	}

	pub fn write(&mut self, value: u8) {
		self.0 = value & !Self::MODE_MASK;
	}

	pub fn int_enable(&self, mode: PPUMode) -> bool {
		match mode {
			PPUMode::Draw => false,
			PPUMode::HBlank => self.has(Self::H_BLANK_IE),
			PPUMode::VBlank => self.has(Self::V_BLANK_IE),
			PPUMode::OamScan => self.has(Self::OAM_IE),
		}
	}

	pub fn lyc_eq_ly(&self) -> bool {
		self.has(Self::LYC_EQ_LY)
	}

	pub fn lyc_eq_ly_ie(&self) -> bool {
		self.has(Self::LYC_EQ_LY_IE)
	}

	pub fn set_lyc_eq_ly(&mut self, value: bool) {
		self.set(Self::LYC_EQ_LY, value);
	}
}
