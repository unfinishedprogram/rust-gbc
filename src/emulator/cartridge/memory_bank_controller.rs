use serde::Serialize;

use crate::emulator::{cartridge::mbc3, memory_mapper::MemoryMapper};

use super::{
	cartridge_data::CartridgeData,
	header::{CartridgeParseError, RawCartridgeHeader},
	mbc1::MBC1State,
	mbc2::MBC2State,
	mbc3::MBC3State,
};

pub trait MemoryBankController: Default + Clone {
	fn read(&mut self, addr: u16) -> u8;
	fn write(&mut self, addr: u16, value: u8);
}

#[derive(Clone, Serialize)]
pub enum Cartridge {
	ROM(CartridgeData),
	MBC1(CartridgeData, MBC1State),
	MBC2(CartridgeData, MBC2State),
	MBC3(CartridgeData, MBC3State),
	MMM01,
	MBC5,
	MBC6,
	MBC7,
	HUC3,
	HUC1,
}

impl TryFrom<&[u8]> for Cartridge {
	type Error = CartridgeParseError;
	fn try_from(value: &[u8]) -> Result<Self, CartridgeParseError> {
		let raw_header = RawCartridgeHeader::from(value);
		let info = raw_header.parse()?;
		log::error!("{info:?}");

		let data = CartridgeData::new(value, info.rom_banks, info.ram_banks);
		use Cartridge::*;

		match raw_header.cartridge_type {
			0x00 => Ok(ROM(data)),
			0x01 | 0x02 | 0x03 => Ok(MBC1(data, MBC1State::default())),
			0x05 | 0x06 => Ok(MBC2(data, MBC2State::default())),
			0x08 | 0x09 => Ok(ROM(data)),
			0x0F | 0x10 | 0x11 | 0x12 | 0x13 => Ok(MBC3(data, MBC3State::default())),
			0x0B | 0x0C | 0x0D => Ok(MMM01),
			0x19 | 0x1A | 0x1B | 0x1C | 0x1D | 0x1E => Ok(MBC5),
			0x20 => Ok(MBC6),
			0x22 => Ok(MBC7),
			0xFE => Ok(HUC3),
			0xFF => Ok(HUC1),
			_ => Err(CartridgeParseError::MBCType),
		}
	}
}

impl MemoryMapper for Cartridge {
	fn read(&self, addr: u16) -> u8 {
		use Cartridge::*;

		match self {
			ROM(data) => match addr {
				0..0x4000 => data.rom_banks[0][addr as usize],
				0x4000..0x8000 => data.rom_banks[1][(addr as usize) - 0x4000],
				_ => unreachable!(),
			},
			MBC1(data, state) => match addr {
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
			MBC2(data, state) => match addr {
				0..0x4000 => data.rom_banks[0][addr as usize],
				0x4000..0x8000 => {
					let bank = state.rom_bank;
					data.rom_banks[bank as usize][(addr - 0x4000) as usize]
				}
				0xA000..0xC000 => {
					let local_addr = ((addr - 0xA000) % 512) as usize;
					if state.ram_enabled {
						state.ram_data[local_addr] & 0x0F
					} else {
						0x0F
					}
				}
				_ => 0xFF,
			},
			MBC3(data, state) => match addr {
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
				_ => 0xFF,
			},
			MMM01 => todo!(),
			MBC5 => todo!(),
			MBC6 => todo!(),
			MBC7 => todo!(),
			HUC3 => todo!(),
			HUC1 => todo!(),
		}
	}

	fn write(&mut self, addr: u16, value: u8) {
		use Cartridge::*;

		match self {
			ROM(_) => {}
			MBC1(data, state) => match addr {
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
			MBC2(_, state) => match addr {
				0x0000..0x4000 => state.set_register(addr, value),
				0xA000..0xC000 => {
					let local_addr = ((addr - 0xA000) % 512) as usize;
					if state.ram_enabled {
						state.ram_data[local_addr] = value & 0x0F
					}
				}
				_ => {}
			},
			MBC3(data, state) => match addr {
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
			MMM01 => {}
			MBC5 => {}
			MBC6 => {}
			MBC7 => {}
			HUC3 => {}
			HUC1 => {}
		}
	}
}
