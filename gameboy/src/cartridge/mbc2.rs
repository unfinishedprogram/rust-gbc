use serde::{Deserialize, Serialize};

use super::cartridge_data::CartridgeData;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MBC2State {
	ram_enabled: bool,
	ram_data: Vec<u8>,
	rom_bank: u8,
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
	fn set_register(&mut self, addr: u16, value: u8) {
		if (1 << 8) & addr == 0 {
			self.ram_enabled = value & 0xF == 0x0A;
		} else {
			self.rom_bank = value & 0x0F;
			if self.rom_bank == 0 {
				self.rom_bank = 1;
			}
		}
	}

	pub fn read(&self, data: &CartridgeData, addr: u16) -> u8 {
		match addr {
			0..0x4000 => data.rom_banks[0][addr as usize],
			0x4000..0x8000 => {
				let bank = self.rom_bank;
				data.rom_banks[bank as usize % data.rom_banks.len()][(addr - 0x4000) as usize]
			}
			0xA000..0xC000 => {
				let local_addr = ((addr - 0xA000) % 512) as usize;
				if self.ram_enabled {
					self.ram_data[local_addr] | 0xF0
				} else {
					0xFF
				}
			}
			_ => unreachable!(),
		}
	}

	pub fn write(&mut self, _data: &mut CartridgeData, addr: u16, value: u8) {
		match addr {
			0x0000..0x4000 => self.set_register(addr, value),
			0xA000..0xC000 => {
				let local_addr = ((addr - 0xA000) % 512) as usize;
				if self.ram_enabled {
					self.ram_data[local_addr] = value | 0xF0
				}
			}
			_ => {}
		}
	}
}
