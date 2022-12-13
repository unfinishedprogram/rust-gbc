use log::warn;

use super::{
	io_registers::IORegisters,
	ppu::{PPUMode, PPU},
	EmulatorState,
};

pub trait MemoryMapper {
	fn read(&self, addr: u16) -> u8;
	fn write(&mut self, addr: u16, value: u8);
}

impl MemoryMapper for EmulatorState {
	fn read(&self, addr: u16) -> u8 {
		match addr {
			0x0000..0x8000 => {
				let Some(rom) = &self.cartridge_state else { return 0 };
				rom.read(addr)
			} // Cartridge Rom
			0x8000..0xA000 => self.v_ram[0][(addr - 0x8000) as usize], //  VRAM
			0xA000..0xC000 => {
				if let Some(rom) = &self.cartridge_state {
					rom.read(addr)
				} else {
					0
				}
			} //  Cartage RAM
			0xC000..0xD000 => self.w_ram[0][(addr - 0xC000) as usize], // Internal RAM
			0xD000..0xE000 => self.w_ram[1][(addr - 0xD000) as usize], // Switchable RAM in CGB mode
			0xE000..0xFE00 => self.read(addr - 0xE000 + 0xC000),       // Mirror, should not be used
			0xFE00..0xFEA0 => self.oam[(addr - 0xFE00) as usize],      // Object Attribute Map
			0xFEA0..0xFF00 => 0x0,                                     // Unusable
			0xFF00..0xFF80 => self.read_io(addr),                      // IO Registers
			0xFF80..0xFFFF => self.hram[(addr - 0xFF80) as usize],     // HRAM
			0xFFFF => self.interrupt_enable_register,                  // Interrupt enable
		}
	}

	fn write(&mut self, addr: u16, value: u8) {
		match addr {
			0x0000..0x8000 => {
				if let Some(rom) = &mut self.cartridge_state {
					rom.write(addr, value);
				}
			} // Cartridge Rom
			0x8000..0xA000 => self.v_ram[0][(addr - 0x8000) as usize] = value, // VRAM
			0xA000..0xC000 => {
				if let Some(rom) = &mut self.cartridge_state {
					rom.write(addr, value);
				} else {
					warn!("Writing to vram bank without rom: {value:X}")
				}
			} // Cartage RAM
			0xC000..0xD000 => self.w_ram[0][(addr - 0xC000) as usize] = value, // Internal RAM
			0xD000..0xE000 => self.w_ram[1][(addr - 0xD000) as usize] = value, // Switchable RAM in CGB mode
			0xE000..0xFE00 => self.write(addr - 0xE000 + 0xC000, value),       // Mirror, should not be used
			0xFE00..0xFEA0 => {
				if matches!(self.get_mode(), PPUMode::VBlank | PPUMode::HBlank) {
					warn!("OAM Write {addr:X}:{value:X}");
					self.oam[(addr - 0xFE00) as usize] = value;
				}
			} // Object Attribute Map
			0xFEA0..0xFF00 => warn!("Invalid Write {addr:X}:{value:X}"),
			0xFF00..0xFF80 => self.write_io(addr, value), // IO Registers
			0xFF80..0xFFFF => self.hram[(addr - 0xFF80) as usize] = value, // HRAM
			0xFFFF => self.interrupt_enable_register = value, // Interrupt enable
		}
	}
}
