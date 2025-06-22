use serde::{Deserialize, Serialize};

use super::cartridge_data::CartridgeData;

#[derive(Clone, Serialize, Deserialize, Debug)]
enum BankingMode {
	Ram,
	Rtc,
}

impl Default for BankingMode {
	fn default() -> Self {
		Self::Ram
	}
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct MBC3State {
	banking_mode: BankingMode,
	rom_bank: usize,
	ram_bank: usize,
	ram_enabled: bool,
	rtc_register: usize,
}

impl MBC3State {
	fn get_rom_bank(&self) -> usize {
		if self.rom_bank == 0 {
			1
		} else {
			self.rom_bank
		}
	}

	fn write_register(&mut self, value: u8) {
		match value {
			0..4 => {
				self.banking_mode = BankingMode::Ram;
				self.ram_bank = value as usize;
			}
			8..0xC => {
				self.banking_mode = BankingMode::Rtc;
				self.rtc_register = value as usize;
			}
			_ => {}
		}
	}

	pub fn read(&self, data: &CartridgeData, addr: u16) -> u8 {
		match addr {
			0..0x4000 => data.rom_banks[0][addr as usize],
			0x4000..0x8000 => {
				let bank = self.get_rom_bank();
				data.rom_banks[bank][(addr - 0x4000) as usize]
			}
			0xA000..0xC000 => match self.banking_mode {
				BankingMode::Ram => data.ram_banks[self.ram_bank][(addr - 0xA000) as usize],
				BankingMode::Rtc => 0,
			},
			_ => unreachable!(),
		}
	}

	pub fn write(&mut self, data: &mut CartridgeData, addr: u16, value: u8) {
		match addr {
			0..0x2000 => self.ram_enabled = value == 0x0A,
			0x2000..0x4000 => self.rom_bank = value as usize,
			0x4000..0x6000 => self.write_register(value),
			0x6000..0x8000 => {}
			0xA000..0xC000 => match self.banking_mode {
				BankingMode::Ram => data.ram_banks[self.ram_bank][(addr - 0xA000) as usize] = value,
				BankingMode::Rtc => {}
			},
			_ => {}
		}
	}
}
