#[derive(Clone)]
enum BankingMode {
	Rom,
	Ram,
}

impl Default for BankingMode {
	fn default() -> Self {
		Self::Rom
	}
}

#[derive(Clone, Default)]
pub struct MBC1State {
	banking_mode: BankingMode,
	banking_register: u8,
	ram_enabled: bool,
}

impl MBC1State {
	pub fn get_rom_bank(&self) -> u16 {
		let bank_mask = match &self.banking_mode {
			BankingMode::Rom => 0b01111111,
			BankingMode::Ram => 0b00011111,
		};
		let bank = self.banking_register & bank_mask;

		(if bank & 0b00011111 == 0 { 1 } else { bank }) as u16
	}

	pub fn get_ram_bank(&self) -> u16 {
		(match &self.banking_mode {
			BankingMode::Rom => 0,
			BankingMode::Ram => (self.banking_register >> 5) & 0b11,
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
		self.banking_mode = match value & 1 == 1 {
			true => BankingMode::Ram,
			false => BankingMode::Rom,
		};
	}
}
