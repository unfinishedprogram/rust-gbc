use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::util::bits::*;

use super::PPUMode;

bitflags! {
	#[derive(Serialize, Deserialize, Default)]
	struct Flags:u8 {
		const PPU_MODE_MASK = BIT_0 | BIT_1;
		const LYC_EQ_LY = BIT_2;
		const H_BLANK_IE = BIT_3;
		const V_BLANK_IE = BIT_4;
		const OAM_IE = BIT_5;
		const LYC_EQ_LY_IE = BIT_6;
		const READ_ONLY = Self::PPU_MODE_MASK.bits | Self::LYC_EQ_LY.bits;
	}
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Stat {
	flags: Flags,
}

impl Stat {
	pub fn set_ppu_mode(&mut self, mode: PPUMode) {
		self.flags.remove(Flags::PPU_MODE_MASK);
		self.flags.bits |= (mode as u8) & Flags::PPU_MODE_MASK.bits;
	}

	pub fn ppu_mode(&self) -> PPUMode {
		let stat = self.flags & Flags::PPU_MODE_MASK;

		match stat.bits {
			0 => PPUMode::HBlank,
			1 => PPUMode::VBlank,
			2 => PPUMode::OamScan,
			3 => PPUMode::Draw,
			_ => unreachable!(),
		}
	}

	pub fn read(&self) -> u8 {
		self.flags.bits
	}

	pub fn write(&mut self, value: u8) {
		let mut value = Flags::from_bits_truncate(value);
		value.remove(Flags::READ_ONLY);
		self.flags.remove(!Flags::READ_ONLY);
		self.flags |= value;
	}

	pub fn int_enable(&self, mode: PPUMode) -> bool {
		match mode {
			PPUMode::Draw => false,
			PPUMode::HBlank => self.flags.contains(Flags::H_BLANK_IE),
			PPUMode::VBlank => self.flags.contains(Flags::V_BLANK_IE),
			PPUMode::OamScan => self.flags.contains(Flags::OAM_IE),
		}
	}

	pub fn lyc_eq_ly(&self) -> bool {
		self.flags.contains(Flags::LYC_EQ_LY)
	}

	pub fn lyc_eq_ly_ie(&self) -> bool {
		self.flags.contains(Flags::LYC_EQ_LY_IE)
	}

	// Returns true if an interrupt should be triggered
	pub fn set_lyc_eq_ly(&mut self, value: bool) -> bool {
		let last = self.flags.contains(Flags::LYC_EQ_LY);
		self.flags.set(Flags::LYC_EQ_LY, value);

		// If the value goes from low to high and Lyc interrupts are enabled, trigger one
		value & last & self.flags.contains(Flags::LYC_EQ_LY_IE)
	}
}
