use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MBC5State {
	rom_bank: u16,
	ram_bank: u8,
	pub ram_enabled: bool,
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
	pub fn get_rom_bank(&self) -> u16 {
		self.rom_bank
	}

	pub fn get_ram_bank(&self) -> u16 {
		self.ram_bank as u16
	}

	pub fn set_ram_bank(&mut self, value: u8) {
		self.ram_bank = value & 0x0F;
	}

	pub fn set_rom_bank(&mut self, value: u8) {
		self.rom_bank &= 0xFF00;
		self.rom_bank |= value as u16;
	}

	pub fn set_rom_bank_upper(&mut self, value: u8) {
		self.rom_bank &= 0xFF;
		self.rom_bank |= (value as u16 & 1) << 8;
	}

	pub fn set_ram_enable(&mut self, value: u8) {
		self.ram_enabled = value & 0xF == 0xA;
	}
}
