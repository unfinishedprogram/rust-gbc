// https://gbdev.io/pandocs/The_Cartridge_Header.html
use wasm_bindgen::{prelude::wasm_bindgen};

#[wasm_bindgen]
#[derive(Debug)]
pub struct Header {
	entry_point: [u8;4], // 0100-0103
	nintendo_logo:[u8;48], // 0104-0133
	title: [u8;16], // 0134-0143
	cgb_flag:u8, // 0143
	license_code: u16, // 0144-0145
	sgb_flag: u8, // 0146
	cartridge_type: u8, // 0147
	rom_size: u8, // 0148
	ram_size: u8, // 0149
	destination_code: u8, // 014A
	old_license_code: u8, // 014B
	mask_rom_version_number: u8, // 014C
	header_checksum: u8, // 014D
	global_checksum: u16, // 014E-014F
}

impl Header {
	pub fn from(rom:&[u8]) -> Header {
		let mut header = Header {
			entry_point: [0;4], // 0100-0103
			nintendo_logo:[0;48], // 0104-0133
			title: [0;16], // 0134-0143
			cgb_flag: rom[0x0143], // 0143
			license_code: ((rom[0x0144] as u16) << 8) | rom[0x0145] as u16, // 0144-0145
			sgb_flag: rom[0x0146], // 0146
			cartridge_type: rom[0x0147], // 0147
			rom_size: rom[0x0148], // 0148
			ram_size: rom[0x0149], // 0149
			destination_code: rom[0x014A], // 014A
			old_license_code: rom[0x014B], // 014B
			mask_rom_version_number: rom[0x014C], // 014C
			header_checksum: rom[0x014D], // 014D
			global_checksum: ((rom[0x014E] as u16) << 8) | rom[0x014F] as u16, // 014E-014F
		};
		
		header.entry_point.copy_from_slice(&rom[0x0100..0x0104]);
		header.nintendo_logo.copy_from_slice(&rom[0x0104..0x0134]);
		header.title.copy_from_slice(&rom[0x0134..0x0144]);

		return header;
	}
}

