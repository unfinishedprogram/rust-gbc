use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CartridgeData {
	pub ram_banks: Vec<Vec<u8>>,

	// Rom banks are initialized as empty, Then they are loaded form network
	#[serde(skip)]
	pub rom_banks: Vec<Vec<u8>>,

	#[serde(skip)]
	pub loaded: bool, // This should be false after deserialization
}

impl CartridgeData {
	pub fn create_rom_banks(banks: u32, raw_data: &[u8]) -> Vec<Vec<u8>> {
		let mut banks: Vec<Vec<u8>> = (0..banks).map(|_| vec![0; 0x4000]).collect();
		// Populate rom banks
		for bank in 0..banks.len() {
			for i in 0..0x4000 {
				banks[bank][i] = raw_data[i + bank * 0x4000]
			}
		}
		banks
	}

	fn create_ram_banks(banks: u32) -> Vec<Vec<u8>> {
		(0..banks).map(|_| vec![0; 0x2000]).collect()
	}

	pub fn new(raw_data: &[u8], rom_banks: impl Into<u32>, ram_banks: impl Into<u32>) -> Self {
		let rom_banks = Self::create_rom_banks(rom_banks.into(), raw_data);
		let ram_banks = Self::create_ram_banks(ram_banks.into());

		Self {
			rom_banks,
			ram_banks,
			loaded: true,
		}
	}
}
