use cartridge_data::CartridgeData;
use header::{CartridgeInfo, CartridgeParseError, RawCartridgeHeader};
use mbc1::MBC1State;
use mbc2::MBC2State;
use mbc3::MBC3State;
use mbc5::MBC5State;
use memory_bank_controller::Mbc;
use serde::{Deserialize, Serialize};

use crate::save_state::RomSource;

mod cartridge_data;
mod header;
mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;
pub mod memory_bank_controller;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Cartridge(pub CartridgeData, pub Mbc, pub CartridgeInfo);

impl Cartridge {
	pub fn try_new(value: &[u8], source: Option<RomSource>) -> Result<Self, CartridgeParseError> {
		let raw_header = RawCartridgeHeader::new(value, source);
		use Mbc::*;
		let info = raw_header.parse()?;
		let data = CartridgeData::new(value, info.rom_banks, info.ram_banks);

		let mbc = match raw_header.cartridge_type {
			0x00 => Ok(ROM),
			0x01..=0x03 => Ok(MBC1(MBC1State::default())),
			0x05 | 0x06 => Ok(MBC2(MBC2State::default())),
			0x08 | 0x09 => Ok(ROM),
			0x0F..=0x13 => Ok(MBC3(MBC3State::default())),
			0x0B..=0x0D => Ok(MMM01),
			0x19..=0x1E => Ok(MBC5(MBC5State::default())),
			0x20 => Ok(MBC6),
			0x22 => Ok(MBC7),
			0xFE => Ok(HUC3),
			0xFF => Ok(HUC1),
			_ => Err(CartridgeParseError::MBCType),
		}?;

		Ok(Cartridge(data, mbc, info))
	}
}
