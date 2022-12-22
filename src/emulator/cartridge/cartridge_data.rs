#[derive(Clone)]
pub struct CartridgeData {
	pub rom_banks: Vec<[u8; 0x4000]>,
	pub ram_banks: Vec<[u8; 0x2000]>,
}

impl CartridgeData {
	fn create_rom_banks(banks: u32) -> Vec<[u8; 0x4000]> {
		(0..banks).map(|_| [0; 0x4000]).collect()
	}

	fn create_ram_banks(banks: u32) -> Vec<[u8; 0x2000]> {
		(0..banks).map(|_| [0; 0x2000]).collect()
	}

	pub fn new(raw_data: &[u8], rom_banks: impl Into<u32>, ram_banks: impl Into<u32>) -> Self {
		let mut rom_banks = Self::create_rom_banks(rom_banks.into());
		let ram_banks = Self::create_ram_banks(ram_banks.into());

		// Populate rom banks
		for bank in 0..rom_banks.len() {
			for i in 0..0x4000 {
				rom_banks[bank][i] = raw_data[i + bank * 0x4000]
			}
		}

		Self {
			rom_banks,
			ram_banks,
		}
	}
}
