use serde::{Deserialize, Serialize};

use super::cartridge_data::CartridgeData;

#[derive(Clone, Serialize, Deserialize, Debug)]
enum BankingMode {
	Simple,
	Complex,
}

impl Default for BankingMode {
	fn default() -> Self {
		Self::Simple
	}
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct MBC1State {
	banking_mode: BankingMode,
	banking_register: u8,
	ram_enabled: bool,
}

impl MBC1State {
	fn get_zero_rom_bank(&self) -> u16 {
		let bank = self.banking_register;
		match self.banking_mode {
			BankingMode::Simple => 0,
			BankingMode::Complex => (bank as u16) & 0b01100000,
		}
	}

	fn get_rom_bank(&self) -> u16 {
		let bank = self.banking_register & 0b01111111;
		(if bank & 0b00011111 == 0 {
			bank + 1
		} else {
			bank
		}) as u16
	}

	fn get_ram_bank(&self) -> u16 {
		(match &self.banking_mode {
			BankingMode::Simple => 0,
			BankingMode::Complex => (self.banking_register >> 5) & 0b11,
		} as u16)
	}

	fn set_ram_bank(&mut self, value: u8) {
		let value = value & 0b00000011;
		self.banking_register &= 0b00011111;
		self.banking_register |= value << 5;
	}

	fn set_rom_bank(&mut self, value: u8) {
		let value = value & 0b00011111;
		self.banking_register &= 0b01100000;
		self.banking_register |= value;
	}

	fn set_ram_enable(&mut self, value: u8) {
		self.ram_enabled = value & 0xF == 0xA;
	}

	fn set_banking_mode(&mut self, value: u8) {
		self.banking_mode = match value == 1 {
			true => BankingMode::Complex,
			false => BankingMode::Simple,
		};
	}

	pub fn read(&self, data: &CartridgeData, addr: u16) -> u8 {
		match addr {
			0..0x4000 => {
				data.rom_banks[self.get_zero_rom_bank() as usize % data.rom_banks.len()]
					[addr as usize]
			}
			0x4000..0x8000 => {
				let bank = self.get_rom_bank() % data.rom_banks.len() as u16;
				data.rom_banks[bank as usize][(addr - 0x4000) as usize]
			}
			0xA000..0xC000 => {
				if data.ram_banks.is_empty() || !self.ram_enabled {
					return 0xFF;
				}
				let bank = self.get_ram_bank() % data.ram_banks.len() as u16;
				data.ram_banks[bank as usize][(addr - 0xA000) as usize]
			}
			_ => unreachable!(),
		}
	}

	pub fn write(&mut self, data: &mut CartridgeData, addr: u16, value: u8) {
		match addr {
			0..0x2000 => self.set_ram_enable(value),
			0x2000..0x4000 => self.set_rom_bank(value),
			0x4000..0x6000 => self.set_ram_bank(value),
			0x6000..0x8000 => self.set_banking_mode(value),
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
