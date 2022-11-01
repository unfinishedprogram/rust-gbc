use std::rc::Rc;

use crate::app::components::logger;

use self::header::{CartridgeInfo, RawCartridgeHeader};

use super::memory_mapper::MemoryMapper;

pub mod header;
pub mod mbc;

#[derive(Clone)]
pub struct CartridgeState {
	pub info: CartridgeInfo,
	pub raw_data: Vec<u8>,
	pub selected_ram_bank: u16,
	pub selected_rom_bank: u16,
	raw_ram: Vec<u8>,
}

impl CartridgeState {
	pub fn from_raw_rom(raw_data: Vec<u8>) -> Result<Self, String> {
		let raw_header = RawCartridgeHeader::from(&raw_data);
		if let Ok(info) = raw_header.parse() {
			Ok(Self {
				raw_ram: vec![0; (info.ram_banks * 0x2000) as usize],
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
			MBC1 => match addr {
				0x0000..0x4000 => self.raw_data[addr as usize], // bank 0
				0x4000..0x8000 => self.raw_data[(addr + 0x4000 * self.selected_rom_bank) as usize], // Bank X
				0xA000..0xC000 => {
					if self.info.ram_banks > 0 {
						self.raw_ram[(addr - 0xA000 + 0x2000 * self.selected_ram_bank) as usize]
					} else {
						0
					}
				}
				_ => {
					logger::error(format!("MBC1 UnhandledRead: {:X}", addr));
					0
				}
			},
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
			MBC1 => match addr {
				2000..4000 => {
					self.selected_rom_bank = (value & 0b00011111) as u16;
					logger::info(format!("Rom Bank:{:} selected", self.selected_rom_bank));
				}
				_ => {}
			},
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
