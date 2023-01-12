use super::{
	io_registers::{IORegisters, DMA},
	state::GameboyMode,
	Gameboy,
};

/// Allows reading and writing to memory using a 16 bit address
pub trait MemoryMapper {
	fn read(&self, addr: u16) -> u8;
	fn write(&mut self, addr: u16, value: u8);
}

/// Similar to `MemoryMapper` but allows specifying a source,
/// This is needed for accurate emulation,
pub trait SourcedMemoryMapper {
	fn read_from(&self, addr: u16, source: Source) -> u8;
	fn write_from(&mut self, addr: u16, value: u8, source: Source);
}

/// Defines a source for a given read/write
pub enum Source {
	/// From the CPU
	Cpu,

	/// From the PPI
	Ppu,

	/// No source, useful for debugging
	Raw,
}

impl SourcedMemoryMapper for Gameboy {
	fn read_from(&self, addr: u16, source: Source) -> u8 {
		if matches!(source, Source::Cpu)
			&& self.dma_timer > 0
			&& !matches!(addr, 0xFF80..0xFFFF)
			&& addr != DMA
		{
			return 0xFF;
		}

		self.read(addr)
	}

	fn write_from(&mut self, addr: u16, value: u8, source: Source) {
		// Don't allow reading from memory outside of HRAM from CPU during DMA transfer
		if matches!(source, Source::Cpu)
			&& self.dma_timer > 0
			&& !matches!(addr, 0xFF80..0xFFFF)
			&& addr != DMA
		{
			return;
		}

		self.write(addr, value)
	}
}

impl MemoryMapper for Gameboy {
	fn read(&self, addr: u16) -> u8 {
		if self.booting {
			match self.mode {
				GameboyMode::DMG => {
					if matches!(addr, 0..=0x100) {
						return self.boot_rom[addr as usize];
					}
				}
				GameboyMode::GBC(_) => match addr {
					0..0x100 => return self.boot_rom[addr as usize],
					0x200..0x900 => return self.boot_rom[addr as usize],
					_ => {}
				},
			}
		}
		match addr {
			0x0000..0x8000 => {
				let Some(rom) = &self.cartridge_state else { return 0xFF };
				rom.read(addr)
			} // Cartridge Rom
			0x8000..0xA000 => self.ppu.v_ram[self.get_vram_bank()][(addr - 0x8000) as usize], //  VRAM
			0xA000..0xC000 => {
				let Some(rom) = &self.cartridge_state else { return 0xFF };
				rom.read(addr)
			} //  Cartage RAM
			0xC000..0xD000 => self.w_ram[0][(addr - 0xC000) as usize],                        // Internal RAM
			0xD000..0xE000 => self.w_ram[self.get_wram_bank()][(addr - 0xD000) as usize], // Switchable RAM in CGB mode
			0xE000..0xFE00 => self.read(addr - 0xE000 + 0xC000), // Mirror, should not be used
			0xFE00..0xFEA0 => self.ppu.oam[(addr - 0xFE00) as usize], // Object Attribute Map
			0xFEA0..0xFF00 => 0x0,                               // Unusable
			0xFF00..0xFF80 => self.read_io(addr),                // IO Registers
			0xFF80..0xFFFF => self.hram[(addr - 0xFF80) as usize], // HRAM
			0xFFFF => self.interrupt_enable_register,            // Interrupt enable
		}
	}

	fn write(&mut self, addr: u16, value: u8) {
		match addr {
			// Cartridge Rom
			0x0000..0x8000 => {
				if let Some(rom) = &mut self.cartridge_state {
					rom.write(addr, value);
				}
			}
			// VRAM
			0x8000..0xA000 => {
				let bank = self.get_vram_bank();
				self.ppu.v_ram[bank][(addr - 0x8000) as usize] = value
			}
			// Cartage RAM
			0xA000..0xC000 => {
				if let Some(rom) = &mut self.cartridge_state {
					rom.write(addr, value);
				}
			}
			0xC000..0xD000 => self.w_ram[0][(addr - 0xC000) as usize] = value, // Internal RAM
			0xD000..0xE000 => {
				let bank = self.get_wram_bank();
				self.w_ram[bank][(addr - 0xD000) as usize] = value
			} // Switchable RAM in CGB mode
			0xE000..0xFE00 => self.write(addr - 0xE000 + 0xC000, value),       // Mirror, should not be used
			0xFE00..0xFEA0 => {
				self.ppu.oam[(addr - 0xFE00) as usize] = value;

				// if matches!(self.get_mode(), PPUMode::VBlank | PPUMode::HBlank) {
				// warn!("OAM Write {addr:X}:{value:X}");
				// self.oam[(addr - 0xFE00) as usize] = value;
				// } else {
				// warn!("BLOCKED OAM Write {addr:X}:{value:X}");
				// self.oam[(addr - 0xFE00) as usize] = value;
				// }
			} // Object Attribute Map
			0xFEA0..0xFF00 => {}

			0xFF00..0xFF80 => self.write_io(addr, value), // IO Registers
			0xFF80..0xFFFF => self.hram[(addr - 0xFF80) as usize] = value, // HRAM
			0xFFFF => self.interrupt_enable_register = value & 0b00011111, // Interrupt enable
		}
	}
}
