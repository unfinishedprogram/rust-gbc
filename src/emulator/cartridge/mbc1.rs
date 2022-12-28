use serde::Serialize;

#[derive(Clone, Serialize)]
enum BankingMode {
	Simple,
	Complex,
}

impl Default for BankingMode {
	fn default() -> Self {
		Self::Simple
	}
}

#[derive(Clone, Default, Serialize)]
pub struct MBC1State {
	banking_mode: BankingMode,
	banking_register: u8,
	pub ram_enabled: bool,
}

impl MBC1State {
	pub fn get_zero_rom_bank(&self) -> u16 {
		let bank = self.banking_register;
		match self.banking_mode {
			BankingMode::Simple => 0,
			BankingMode::Complex => (bank as u16) & 0b01100000,
		}
	}

	pub fn get_rom_bank(&self) -> u16 {
		let bank = self.banking_register & 0b01111111;
		(if bank & 0b00011111 == 0 {
			bank + 1
		} else {
			bank
		}) as u16
	}

	pub fn get_ram_bank(&self) -> u16 {
		(match &self.banking_mode {
			BankingMode::Simple => 0,
			BankingMode::Complex => (self.banking_register >> 5) & 0b11,
		} as u16)
	}

	pub fn set_ram_bank(&mut self, value: u8) {
		let value = value & 0b00000011;
		self.banking_register &= 0b00011111;
		self.banking_register |= value << 5;
	}

	pub fn set_rom_bank(&mut self, value: u8) {
		let value = value & 0b00011111;
		self.banking_register &= 0b01100000;
		self.banking_register |= value;
	}

	pub fn set_ram_enable(&mut self, value: u8) {
		self.ram_enabled = value & 0xF == 0xA;
	}

	pub fn set_banking_mode(&mut self, value: u8) {
		self.banking_mode = match value == 1 {
			true => BankingMode::Complex,
			false => BankingMode::Simple,
		};
	}
}
