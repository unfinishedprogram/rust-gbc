use serde::{Deserialize, Serialize};
use sm83::memory_mapper::MemoryMapper;

use super::{mbc1::MBC1State, mbc2::MBC2State, mbc3::MBC3State, mbc5::MBC5State, Cartridge};

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

impl MemoryMapper for Cartridge {
	fn read(&self, addr: u16) -> u8 {
		use Mbc::*;
		let Cartridge(data, mbc, _info) = self;
		match mbc {
			ROM => match addr {
				0..0x4000 => data.rom_banks[0][addr as usize],
				0x4000..0x8000 => data.rom_banks[1][(addr as usize) - 0x4000],
				_ => 0xFF,
			},
			MBC1(state) => state.read(data, addr),
			MBC2(state) => state.read(data, addr),
			MBC3(state) => state.read(data, addr),
			MBC5(state) => state.read(data, addr),
			_ => todo!(),
		}
	}

	fn write(&mut self, addr: u16, value: u8) {
		use Mbc::*;

		let Cartridge(data, mbc, _info) = self;

		match mbc {
			ROM => {}
			MBC1(state) => state.write(data, addr, value),
			MBC2(state) => state.write(data, addr, value),
			MBC3(state) => state.write(data, addr, value),
			MBC5(state) => state.write(data, addr, value),
			_ => todo!(),
		}
	}
}
