use log::error;

use self::header::{CartridgeInfo, CartridgeParseError, RawCartridgeHeader};

use super::memory_mapper::MemoryMapper;

pub mod header;
pub mod mbc;

#[derive(Clone)]
enum MBC1BankingMode {
	Rom,
	Ram,
}
#[derive(Clone)]
pub struct CartridgeState {
	pub info: CartridgeInfo,
	pub raw_data: Vec<u8>,
	pub bank_register: u8,

	raw_ram: Vec<u8>,

	banking_mode: MBC1BankingMode,
	ram_enabled: bool,
}

impl CartridgeState {
	pub fn from_raw_rom(raw_data: Vec<u8>) -> Result<Self, CartridgeParseError> {
		log::error!("Len:{}", raw_data.len());
		let raw_header = RawCartridgeHeader::from(&raw_data);
		let info = raw_header.parse()?;

		Ok(Self {
			ram_enabled: false,
			raw_ram: vec![0; (info.ram_banks * 0x2000) as usize],
			info,
			raw_data,
			banking_mode: MBC1BankingMode::Rom,
			bank_register: 1,
		})
	}

	fn get_rom_bank(&self) -> u16 {
		let bank_mask = match self.banking_mode {
			MBC1BankingMode::Rom => 0b01111111,
			MBC1BankingMode::Ram => 0b00011111,
		};
		let bank = self.bank_register & bank_mask;

		(if bank & 0b00011111 == 0 { 1 } else { bank }) as u16 % self.info.rom_banks
	}

	fn get_ram_bank(&self) -> u16 {
		(match self.banking_mode {
			MBC1BankingMode::Rom => 0,
			MBC1BankingMode::Ram => ((self.bank_register >> 5) & 0b11) % self.info.ram_banks as u8,
		} as u16)
	}

	fn set_ram_bank(&mut self, value: u8) {
		let value = value & 0b00000011;
		self.bank_register &= 0b00011111;
		self.bank_register |= value << 5;
	}

	fn set_rom_bank(&mut self, value: u8) {
		let value = value & 0b00011111;
		self.bank_register &= 0b01100000;
		self.bank_register |= value;
	}
}

impl MemoryMapper for CartridgeState {
	fn read(&self, addr: u16) -> u8 {
		use mbc::MBC::*;

		match self.info.mbc {
			ROM => self.raw_data[addr as usize],

			MBC1 => match addr {
				0x0000..0x4000 => self.raw_data[addr as usize], // bank 0
				0x4000..0x8000 => {
					let base = addr - 0x4000;
					let raw_offset = 0x4000 * self.get_rom_bank();
					self.raw_data[(base + raw_offset) as usize]
				} // Bank X
				0xA000..0xC000 => {
					if self.info.ram_banks > 0 && self.ram_enabled {
						let mapped_addr = 0x2000 * self.get_ram_bank() - 0xA000;
						self.raw_ram[(addr + mapped_addr) as usize]
					} else {
						0xFF
					}
				}
				_ => {
					error!("MBC1 UnhandledRead: {:X}", addr);
					0xFF
				}
			},
			MBC2 => todo!(),
			MMM01 => todo!(),
			MBC3 => todo!(),
			MBC5 => todo!(),
			MBC6 => todo!(),
			MBC7 => todo!(),
			HUC3 => todo!(),
			HUC1 => todo!(),
		}
	}

	fn write(&mut self, addr: u16, value: u8) {
		use mbc::MBC::*;
		match self.info.mbc {
			ROM => {}
			MBC1 => match addr {
				0x0000..0x2000 => self.ram_enabled = value & 0x0F == 0x0A,
				0x2000..0x4000 => {
					self.set_rom_bank(value);
				}
				0x4000..0x6000 => {
					self.set_ram_bank(value);
				}
				0x6000..0x8000 => {
					self.banking_mode = match value & 1 == 1 {
						true => MBC1BankingMode::Ram,
						false => MBC1BankingMode::Rom,
					}
				}
				0xA000..0xC000 => {
					if self.ram_enabled {
						let mapped_addr = 0x2000 * self.get_ram_bank() - 0xA000;
						self.raw_ram[(mapped_addr + addr) as usize] = value;
					}
				}
				_ => {
					log::error!("Writing outide of cartrage")
				}
			},
			MBC2 => todo!(),
			MMM01 => todo!(),
			MBC3 => todo!(),
			MBC5 => todo!(),
			MBC6 => todo!(),
			MBC7 => todo!(),
			HUC3 => todo!(),
			HUC1 => todo!(),
		}
	}
}
