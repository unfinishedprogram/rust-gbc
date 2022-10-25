use super::EmulatorState;

pub trait MemoryMapper {
	fn read(&self, addr: u16) -> u8;
	fn write(&mut self, addr: u16, value: u8);
}

impl MemoryMapper for EmulatorState {
	fn read(&self, addr: u16) -> u8 {
		match addr {
			0x0000..0x8000 => self.cartridge_state.read(addr), // Cartridge Rom
			0x8000..0xA000 => self.v_ram[0][(addr - 0x8000) as usize], //  VRAM
			0xA000..0xC000 => todo!(),                         //  Cartrage RAM
			0xC000..0xD000 => self.w_ram[0][(addr - 0xC000) as usize], // Internal RAM
			0xD000..0xE000 => self.w_ram[1][(addr - 0xD000) as usize], // Switchable RAM in CGB mode
			0xE000..0xFE00 => self.w_ram[0][(addr - 0xE000) as usize], // Mirror, should not be used
			0xFE00..0xFEA0 => self.oam[(addr - 0xFE00) as usize], // Object Attribute Map
			0xFEA0..0xFF00 => 0x0,                             // Unusable
			0xFF00..0xFF80 => todo!("IO Registers"),           // IO Registers
			0xFF80..0xFFFF => self.hram[(addr - 0xFF80) as usize], // HRAM
			0xFFFF => todo!(),                                 // Interupt enable
		}
	}

	fn write(&mut self, addr: u16, value: u8) {
		todo!();
		// match addr {
		// 	0x0000..0x8000 => self.cartridge_state.write(addr, value), // Rom bank 0
		// 	0x8000..0xA000 => self.v_ram[addr - 0x8000],               // VRAM
		// 	0xA000..0xC000 => todo!(),                                 // Cartrage RAM
		// 	0xC000..0xD000 => self.w_ram[0][addr - 0xC000],            // Internal RAM
		// 	0xD000..0xE000 => self.w_ram[1][addr - 0xD000],            // Switchable RAM in CGB mode
		// 	0xE000..0xFE00 => self.w_ram[0][addr - 0xE000],            // Mirror, should not be used
		// 	0xFE00..0xFEA0 => self.oam[addr - 0xFE00],                 // Object Attribute Map
		// 	0xFEA0..0xFF00 => 0x0,                                     // Unusable
		//  0xFF00..0xFF80 => todo!("IO Registers"),           // IO Registers
		// 	0xFF80..0xFFFF => self.hram[addr - 0xFF80],                // HRAM
		// 	0xFFFF => todo!(),                                         // Interupt enable
		// }
	}
}
