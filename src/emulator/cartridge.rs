use std::rc::Rc;

use crate::app::components::logger;

use self::header::{CartridgeInfo, RawCartridgeHeader};

use super::memory_mapper::MemoryMapper;

pub mod header;
pub mod mbc;

pub struct CartridgeState {
	pub info: CartridgeInfo,
	pub raw_data: Vec<u8>,
	pub selected_ram_bank: u16,
	pub selected_rom_bank: u16,
}

impl CartridgeState {
	pub fn from_raw_rom(raw_data: Vec<u8>) -> Result<Self, String> {
		let raw_header = RawCartridgeHeader::from(&raw_data);
		if let Ok(info) = raw_header.parse() {
			Ok(Self {
				info,
				raw_data,
				selected_ram_bank: 1,
				selected_rom_bank: 1,
			})
		} else {
			Err("Parse Error".to_owned())
		}
	}
}

impl MemoryMapper for CartridgeState {
	fn read(&self, addr: u16) -> u8 {
		use mbc::MBC::*;

		match self.info.mbc {
			ROM => self.raw_data[addr as usize],
			MBC1 => todo!(),
			MBC2 => todo!(),
			MMM01 => todo!(),
			MBC3 => todo!(),
			MBC5 => todo!(),
			MBC6 => todo!(),
			MBC7 => todo!(),
			HUC3 => todo!(),
			HUC1 => todo!(),
		}
	}

	fn write(&mut self, addr: u16, value: u8) {
		use mbc::MBC::*;
		match self.info.mbc {
			ROM => {
				logger::warn(format!("Write to readonly memory: {:X}", addr));
			}
			MBC1 => todo!(),
			MBC2 => todo!(),
			MMM01 => todo!(),
			MBC3 => todo!(),
			MBC5 => todo!(),
			MBC6 => todo!(),
			MBC7 => todo!(),
			HUC3 => todo!(),
			HUC1 => todo!(),
		}
	}
}
