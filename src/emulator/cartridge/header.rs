// https://gbdev.io/pandocs/The_Cartridge_Header.html

use super::mbc::MBC;

use crate::app::components::logger;

#[derive(Debug, Clone)]
pub enum CartridgeParseError {}

#[derive(Debug)]
pub struct RawCartridgeHeader {
	pub cgb_flag: u8,                // 0143
	pub license_code: u16,           // 0144-0145
	pub sgb_flag: u8,                // 0146
	pub cartridge_type: u8,          // 0147
	pub rom_size: u8,                // 0148
	pub ram_size: u8,                // 0149
	pub old_license_code: u8,        // 014B
	pub mask_rom_version_number: u8, // 014C
	pub header_checksum: u8,         // 014D
	pub global_checksum: u16,        // 014E-014F
}

#[derive(Debug)]
pub struct CartridgeInfo {
	pub cgb: bool,
	pub sgb: bool,
	pub rom_banks: u16,
	pub ram_banks: u16,
	pub mbc: MBC,
}

impl RawCartridgeHeader {
	fn get_rom_banks(&self) -> u16 {
		if self.rom_size > 0x08 {
			logger::error(format!("Invalid rom size: {:X}", self.rom_size));
			return 2;
		}

		2 >> (self.rom_size + 1)
	}

	fn get_ram_banks(&self) -> u16 {
		match self.ram_size {
			0x00 | 0x01 => 0,
			0x02 => 1,
			0x03 => 4,
			0x04 => 16,
			0x05 => 8,
			value => {
				logger::error(format!("Invalid Rom Size: {:X}", value));
				0
			}
		}
	}

	fn get_mbc(&self) -> MBC {
		use MBC::*;
		match self.cartridge_type {
			0x00 => ROM,
			0x01 | 0x02 | 0x03 => MBC1,
			0x05 | 0x06 => MBC2,
			0x08 | 0x09 => ROM,
			0x0B | 0x0C | 0x0D => MMM01,
			0x0F | 0x10 | 0x11 | 0x12 | 0x13 => MBC3,
			0x19 | 0x1A | 0x1B | 0x1C | 0x1D | 0x1E => MBC5,
			0x20 => MBC6,
			0x22 => MBC7,
			0xFE => HUC3,
			0xFF => HUC1,
			value => {
				logger::error(format!("Cartrage type {:x} not recognized", value));
				ROM
			}
		}
	}

	pub fn parse(&self) -> Result<CartridgeInfo, CartridgeParseError> {
		let cgb = match self.cgb_flag {
			0x80 | 0xC0 => true,
			_ => false,
		};

		let sgb = self.sgb_flag == 0x03;

		Ok(CartridgeInfo {
			cgb,
			sgb,
			rom_banks: self.get_rom_banks(),
			ram_banks: self.get_ram_banks(),
			mbc: self.get_mbc(),
		})
	}
}

impl From<&Vec<u8>> for RawCartridgeHeader {
	fn from(rom: &Vec<u8>) -> Self {
		RawCartridgeHeader {
			cgb_flag: rom[0x0143],                                             // 0143
			license_code: ((rom[0x0144] as u16) << 8) | rom[0x0145] as u16,    // 0144-0145
			sgb_flag: rom[0x0146],                                             // 0146
			cartridge_type: rom[0x0147],                                       // 0147
			rom_size: rom[0x0148],                                             // 0148
			ram_size: rom[0x0149],                                             // 0149
			old_license_code: rom[0x014B],                                     // 014B
			mask_rom_version_number: rom[0x014C],                              // 014C
			header_checksum: rom[0x014D],                                      // 014D
			global_checksum: ((rom[0x014E] as u16) << 8) | rom[0x014F] as u16, // 014E-014F
		}
	}
}

impl Default for CartridgeInfo {
	fn default() -> Self {
		Self {
			cgb: false,
			sgb: false,
			rom_banks: 2,
			ram_banks: 0,
			mbc: MBC::ROM,
		}
	}
}
