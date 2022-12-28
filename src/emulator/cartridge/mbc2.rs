use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct MBC2State {
	pub ram_enabled: bool,
	pub ram_data: Vec<u8>,
	pub rom_bank: u8,
}

impl Default for MBC2State {
	fn default() -> Self {
		Self {
			rom_bank: 1,
			ram_enabled: false,
			ram_data: vec![0; 512],
		}
	}
}

impl MBC2State {
	pub fn set_register(&mut self, addr: u16, value: u8) {
		if addr & 1 << 8 == 0 {
			self.ram_enabled = value & 0xF == 0x0A;
		} else {
			self.rom_bank = value & 0x0F;
			if self.rom_bank == 0 {
				self.rom_bank = 1;
			}
		}
	}
}
