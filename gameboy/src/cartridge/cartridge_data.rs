use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use crate::util::BigArray;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RamBank {
	#[serde(with = "BigArray")]
	pub data: [u8; 0x2000],
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RomBank {
	#[serde(with = "BigArray")]
	pub data: [u8; 0x4000],
}

impl Index<usize> for RomBank {
	type Output = u8;

	fn index(&self, index: usize) -> &Self::Output {
		&self.data[index]
	}
}

impl Index<usize> for RamBank {
	type Output = u8;

	fn index(&self, index: usize) -> &Self::Output {
		&self.data[index]
	}
}

impl IndexMut<usize> for RamBank {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.data[index]
	}
}

impl IndexMut<usize> for RomBank {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.data[index]
	}
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CartridgeData {
	pub ram_banks: Vec<RamBank>,

	// Rom banks are initialized as empty, Then they are loaded form network
	#[serde(skip)]
	pub rom_banks: Vec<RomBank>,

	#[serde(skip)]
	pub loaded: bool, // This should be false after deserialization
}

impl CartridgeData {
	pub fn create_rom_banks(banks: u32, raw_data: &[u8]) -> Vec<RomBank> {
		let mut banks: Vec<RomBank> = (0..banks).map(|_| RomBank { data: [0; 0x4000] }).collect();
		// Populate rom banks
		for bank in 0..banks.len() {
			for i in 0..0x4000 {
				banks[bank].data[i] = raw_data[i + bank * 0x4000]
			}
		}
		banks
	}

	fn create_ram_banks(banks: u32) -> Vec<RamBank> {
		(0..banks).map(|_| RamBank { data: [0; 0x2000] }).collect()
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
