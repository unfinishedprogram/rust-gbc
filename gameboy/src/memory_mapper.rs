use sm83::memory_mapper::{MemoryMapper, Source, SourcedMemoryMapper};

use crate::{
	io_registers::{IORegisters, DMA},
	ppu::{PPUMode, VRAMBank},
	state::GameboyMode,
	work_ram::BankedWorkRam,
	Gameboy,
};

impl SourcedMemoryMapper for Gameboy {
	fn read_from(&self, addr: u16, source: Source) -> u8 {
		if matches!(source, Source::Cpu)
			&& !self.oam_dma.oam_is_accessible()
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
			&& !self.oam_dma.oam_is_accessible()
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
			0x8000..0xA000 => match self.get_vram_bank() {
				VRAMBank::Bank0 => self.ppu.v_ram_bank_0[(addr - 0x8000) as usize],
				VRAMBank::Bank1 => self.ppu.v_ram_bank_1[(addr - 0x8000) as usize],
			}, //  VRAM
			0xA000..0xC000 => {
				let Some(rom) = &self.cartridge_state else { return 0xFF };
				rom.read(addr)
			} //  Cartage RAM
			0xC000..0xD000 => self.w_ram.get_bank(0)[(addr - 0xC000) as usize], // Internal RAM
			0xD000..0xE000 => {
				self.w_ram.get_bank(self.w_ram.get_bank_number())[(addr - 0xD000) as usize]
			} // Switchable RAM in CGB mode
			0xE000..0xFE00 => self.read(addr - 0xE000 + 0xC000),                // Mirror, should not be used
			0xFE00..0xFEA0 => {
				if !matches!(self.ppu.get_mode(), PPUMode::Draw | PPUMode::OamScan) {
					self.ppu.oam[(addr - 0xFE00) as usize]
				} else {
					0xFF
				}
			} // Object Attribute Map
			0xFEA0..0xFF00 => 0x0,                                              // Unusable
			0xFF00..0xFF80 => self.read_io(addr),                               // IO Registers
			0xFF80..0xFFFF => self.hram[(addr - 0xFF80) as usize],              // HRAM
			0xFFFF => self.cpu_state.ie_register,                               // Interrupt enable
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
			0x8000..0xA000 => match self.get_vram_bank() {
				VRAMBank::Bank0 => self.ppu.v_ram_bank_0[(addr - 0x8000) as usize] = value,
				VRAMBank::Bank1 => self.ppu.v_ram_bank_1[(addr - 0x8000) as usize] = value,
			},
			// Cartage RAM
			0xA000..0xC000 => {
				if let Some(rom) = &mut self.cartridge_state {
					rom.write(addr, value);
				}
			}
			0xC000..0xD000 => self.w_ram.get_bank_mut(0)[(addr - 0xC000) as usize] = value, // Internal RAM
			0xD000..0xE000 => {
				let bank = self.w_ram.get_bank_number();
				self.w_ram.get_bank_mut(bank)[(addr - 0xD000) as usize] = value
			} // Switchable RAM in CGB mode
			0xE000..0xFE00 => self.write(addr - 0xE000 + 0xC000, value), // Mirror, should not be used
			0xFE00..0xFEA0 => {
				if matches!(self.ppu.get_mode(), PPUMode::Draw | PPUMode::OamScan) {
					return;
				}
				self.ppu.oam[(addr - 0xFE00) as usize] = value;
			} // Object Attribute Map
			0xFEA0..0xFF00 => {}

			0xFF00..0xFF80 => self.write_io(addr, value), // IO Registers
			0xFF80..0xFFFF => self.hram[(addr - 0xFF80) as usize] = value, // HRAM
			0xFFFF => self.cpu_state.ie_register = value & 0b00011111, // Interrupt enable
		}
	}
}
