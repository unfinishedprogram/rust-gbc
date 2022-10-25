use std::rc::Rc;

use self::header::{CartridgeInfo, RawCartridgeHeader};

use super::memory_mapper::MemoryMapper;

pub mod header;
pub mod mbc;

pub struct CartridgeState {
	pub info: CartridgeInfo,
	pub raw_data: Rc<Vec<u8>>,
	pub selected_ram_bank: u16,
	pub selected_rom_bank: u16,
}

impl CartridgeState {
	pub fn from_raw_rom(raw_data: Rc<Vec<u8>>) -> Result<Self, String> {
		let raw_header = RawCartridgeHeader::from(&*raw_data);
		if let Ok(info) = raw_header.parse() {
			Ok(Self {
				info,
				raw_data,
				selected_ram_bank: 1,
				selected_rom_bank: 1,
			})
		} else {
			Err("Parse Error".to_owned())
		}
	}
}

impl Default for CartridgeState {
	fn default() -> Self {
		Self {
			info: CartridgeInfo::default(),
			raw_data: Rc::new(vec![0; 512]),
			selected_ram_bank: 1,
			selected_rom_bank: 1,
		}
	}
}

impl MemoryMapper for CartridgeState {
	fn read(&self, addr: u16) -> u8 {
		todo!()
	}

	fn write(&mut self, addr: u16, value: u8) {
		todo!()
	}
}
