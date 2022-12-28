use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum BankingMode {
	Ram,
	Rtc,
}

impl Default for BankingMode {
	fn default() -> Self {
		Self::Ram
	}
}

#[derive(Clone, Default, Serialize)]
pub struct MBC3State {
	pub banking_mode: BankingMode,
	pub rom_bank: usize,
	pub ram_bank: usize,
	pub ram_enabled: bool,
	pub rtc_register: usize,
}

impl MBC3State {
	pub fn get_rom_bank(&self) -> usize {
		if self.rom_bank == 0 {
			1
		} else {
			self.rom_bank
		}
	}

	pub fn write_register(&mut self, value: u8) {
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
}
