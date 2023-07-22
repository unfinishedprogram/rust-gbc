use serde::{Deserialize, Serialize};
use sm83::memory_mapper::MemoryMapper;

use crate::{cartridge::mbc3, save_state::RomSource};

use super::{
	cartridge_data::CartridgeData,
	header::{CartridgeInfo, CartridgeParseError, RawCartridgeHeader},
	mbc1::MBC1State,
	mbc2::MBC2State,
	mbc3::MBC3State,
	mbc5::MBC5State,
};

pub trait MemoryBankController: Default + Clone {
	fn read(&mut self, addr: u16) -> u8;
	fn write(&mut self, addr: u16, value: u8);
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Mbc {
	ROM,
	MBC1(MBC1State),
	MBC2(MBC2State),
	MBC3(MBC3State),
	MBC5(MBC5State),
	MBC6,
	MMM01,
	MBC7,
	HUC3,
	HUC1,
}

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
			0x01 | 0x02 | 0x03 => Ok(MBC1(MBC1State::default())),
			0x05 | 0x06 => Ok(MBC2(MBC2State::default())),
			0x08 | 0x09 => Ok(ROM),
			0x0F | 0x10 | 0x11 | 0x12 | 0x13 => Ok(MBC3(MBC3State::default())),
			0x0B | 0x0C | 0x0D => Ok(MMM01),
			0x19 | 0x1A | 0x1B | 0x1C | 0x1D | 0x1E => Ok(MBC5(MBC5State::default())),
			0x20 => Ok(MBC6),
			0x22 => Ok(MBC7),
			0xFE => Ok(HUC3),
			0xFF => Ok(HUC1),
			_ => Err(CartridgeParseError::MBCType),
		}?;

		Ok(Cartridge(data, mbc, info))
	}
}

impl MemoryMapper for Cartridge {
	fn read(&self, addr: u16) -> u8 {
		use Mbc::*;
		let Cartridge(data, mbc, _info) = self;
		match mbc {
			ROM => match addr {
				0..0x4000 => data.rom_banks[0][addr as usize],
				0x4000..0x8000 => data.rom_banks[1][(addr as usize) - 0x4000],
				_ => unreachable!(),
			},
			MBC1(state) => match addr {
				0..0x4000 => {
					data.rom_banks[state.get_zero_rom_bank() as usize % data.rom_banks.len()]
						[addr as usize]
				}
				0x4000..0x8000 => {
					let bank = state.get_rom_bank() % data.rom_banks.len() as u16;
					data.rom_banks[bank as usize][(addr - 0x4000) as usize]
				}
				0xA000..0xC000 => {
					if data.ram_banks.is_empty() || !state.ram_enabled {
						return 0xFF;
					}
					let bank = state.get_ram_bank() % data.ram_banks.len() as u16;
					data.ram_banks[bank as usize][(addr - 0xA000) as usize]
				}

				_ => unreachable!(),
			},
			MBC2(state) => match addr {
				0..0x4000 => data.rom_banks[0][addr as usize],
				0x4000..0x8000 => {
					let bank = state.rom_bank;
					data.rom_banks[bank as usize % data.rom_banks.len()][(addr - 0x4000) as usize]
				}
				0xA000..0xC000 => {
					let local_addr = ((addr - 0xA000) % 512) as usize;
					if state.ram_enabled {
						state.ram_data[local_addr] | 0xF0
					} else {
						0xFF
					}
				}
				_ => unreachable!(),
			},
			MBC3(state) => match addr {
				0..0x4000 => data.rom_banks[0][addr as usize],
				0x4000..0x8000 => {
					let bank = state.get_rom_bank();
					data.rom_banks[bank][(addr - 0x4000) as usize]
				}
				0xA000..0xC000 => match state.banking_mode {
					mbc3::BankingMode::Ram => {
						data.ram_banks[state.ram_bank][(addr - 0xA000) as usize]
					}
					mbc3::BankingMode::Rtc => 0,
				},
				_ => unreachable!(),
			},
			MMM01 => todo!(),
			MBC5(state) => match addr {
				0..0x4000 => data.rom_banks[0][addr as usize],
				0x4000..0x8000 => {
					let bank = state.get_rom_bank() as usize % data.rom_banks.len();
					data.rom_banks[bank][(addr - 0x4000) as usize]
				}
				0xA000..0xC000 => {
					if data.ram_banks.is_empty() || !state.ram_enabled {
						return 0xFF;
					}
					let bank = state.get_ram_bank() as usize % data.ram_banks.len();
					data.ram_banks[bank][(addr - 0xA000) as usize]
				}
				_ => unreachable!(),
			},
			MBC6 => todo!(),
			MBC7 => todo!(),
			HUC3 => todo!(),
			HUC1 => todo!(),
		}
	}

	fn write(&mut self, addr: u16, value: u8) {
		use Mbc::*;

		let Cartridge(data, mbc, _info) = self;

		match mbc {
			ROM => {}
			MBC1(state) => match addr {
				0..0x2000 => state.set_ram_enable(value),
				0x2000..0x4000 => state.set_rom_bank(value),
				0x4000..0x6000 => state.set_ram_bank(value),
				0x6000..0x8000 => state.set_banking_mode(value),
				0xA000..0xC000 => {
					if data.ram_banks.is_empty() || !state.ram_enabled {
						return;
					}
					let bank = state.get_ram_bank() % data.ram_banks.len() as u16;
					data.ram_banks[bank as usize][(addr - 0xA000) as usize] = value;
				}
				_ => unreachable!(),
			},
			MBC2(state) => match addr {
				0x0000..0x4000 => state.set_register(addr, value),
				0xA000..0xC000 => {
					let local_addr = ((addr - 0xA000) % 512) as usize;
					if state.ram_enabled {
						state.ram_data[local_addr] = value | 0xF0
					}
				}
				_ => {}
			},
			MBC3(state) => match addr {
				0..0x2000 => state.ram_enabled = value == 0x0A,
				0x2000..0x4000 => state.rom_bank = value as usize,
				0x4000..0x6000 => state.write_register(value),
				0x6000..0x8000 => {}
				0xA000..0xC000 => match state.banking_mode {
					mbc3::BankingMode::Ram => {
						data.ram_banks[state.ram_bank][(addr - 0xA000) as usize] = value
					}
					mbc3::BankingMode::Rtc => {}
				},
				_ => {}
			},
			MBC5(state) => match addr {
				0..0x2000 => state.set_ram_enable(value),
				0x2000..0x3000 => state.set_rom_bank(value),
				0x3000..0x4000 => state.set_rom_bank_upper(value),
				0x4000..0x6000 => state.set_ram_bank(value),
				0x6000..0x8000 => {}
				0xA000..0xC000 => {
					if data.ram_banks.is_empty() || !state.ram_enabled {
						return;
					}
					let bank = state.get_ram_bank() % data.ram_banks.len() as u16;
					data.ram_banks[bank as usize][(addr - 0xA000) as usize] = value;
				}
				_ => unreachable!(),
			},
			_ => todo!(),
		}
	}
}
