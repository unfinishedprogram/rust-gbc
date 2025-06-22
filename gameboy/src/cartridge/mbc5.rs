use serde::{Deserialize, Serialize};

use super::cartridge_data::CartridgeData;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MBC5State {
	rom_bank: u16,
	ram_bank: u8,
	ram_enabled: bool,
}

impl Default for MBC5State {
	fn default() -> Self {
		Self {
			rom_bank: 1,
			ram_bank: 0,
			ram_enabled: false,
		}
	}
}

impl MBC5State {
	fn get_rom_bank(&self) -> u16 {
		self.rom_bank
	}

	fn get_ram_bank(&self) -> u16 {
		self.ram_bank as u16
	}

	fn set_ram_bank(&mut self, value: u8) {
		self.ram_bank = value & 0x0F;
	}

	fn set_rom_bank(&mut self, value: u8) {
		self.rom_bank &= 0xFF00;
		self.rom_bank |= value as u16;
	}

	fn set_rom_bank_upper(&mut self, value: u8) {
		self.rom_bank &= 0x00FF;
		self.rom_bank |= (value as u16 & 1) << 8;
	}

	fn set_ram_enable(&mut self, value: u8) {
		self.ram_enabled = value & 0xF == 0xA;
	}

	pub fn read(&self, data: &CartridgeData, addr: u16) -> u8 {
		match addr {
			0..0x4000 => data.rom_banks[0][addr as usize],
			0x4000..0x8000 => {
				let bank = self.get_rom_bank() as usize % data.rom_banks.len();
				data.rom_banks[bank][(addr - 0x4000) as usize]
			}
			0xA000..0xC000 => {
				if data.ram_banks.is_empty() || !self.ram_enabled {
					return 0xFF;
				}
				let bank = self.get_ram_bank() as usize % data.ram_banks.len();
				data.ram_banks[bank][(addr - 0xA000) as usize]
			}
			_ => unreachable!(),
		}
	}

	pub fn write(&mut self, data: &mut CartridgeData, addr: u16, value: u8) {
		match addr {
			0..0x2000 => self.set_ram_enable(value),
			0x2000..0x3000 => self.set_rom_bank(value),
			0x3000..0x4000 => self.set_rom_bank_upper(value),
			0x4000..0x6000 => self.set_ram_bank(value),
			0x6000..0x8000 => {}
			0xA000..0xC000 => {
				if data.ram_banks.is_empty() || !self.ram_enabled {
					return;
				}
				let bank = self.get_ram_bank() % data.ram_banks.len() as u16;
				data.ram_banks[bank as usize][(addr - 0xA000) as usize] = value;
			}
			_ => unreachable!(),
		}
	}
}
