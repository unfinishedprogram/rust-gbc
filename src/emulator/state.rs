use std::rc::Rc;

use super::cartridge::CartridgeState;
use super::cpu::CPUState;
use super::memory_mapper::MemoryMapper;
use crate::app::components::logger;

pub struct EmulatorState {
	pub ram_bank: u8,
	pub cgb: bool,
	pub t_states: u64,
	pub cpu_state: CPUState,
	pub cartridge_state: CartridgeState,
	pub v_ram: [[u8; 0x2000]; 2],
	pub w_ram: [[u8; 0x1000]; 8],
	pub oam: [u8; 0xA0],
	pub hram: [u8; 0x80],
	pub interupt_register: u8,
}

impl Default for EmulatorState {
	fn default() -> Self {
		Self {
			cpu_state: CPUState::default(),
			cartridge_state: CartridgeState::default(),
			ram_bank: 0,
			cgb: false,
			t_states: 0,
			interupt_register: 0,
			v_ram: [[0; 0x2000]; 2],
			w_ram: [[0; 0x1000]; 8],
			oam: [0; 0xA0],
			hram: [0; 0x80],
		}
	}
}

impl EmulatorState {
	pub fn step(&mut self) {}

	pub fn load_rom(&mut self, rom: Rc<Vec<u8>>) {
		let rom_data = Rc::new(Box::new(rom));

		if let Ok(state) = CartridgeState::from_raw_rom(Rc::clone(&rom_data)) {
			logger::info("Loaded Rom");
			logger::info(format!("{:?}", state.info));

			self.cartridge_state = state;
		} else {
			logger::error("Rom Loading Failed")
		}
	}
}

impl MemoryMapper for EmulatorState {
	fn read(&self, addr: u16) -> u8 {
		match addr {
			0x0000..0x8000 => self.cartridge_state.read(addr), // Cartridge Rom
			0x8000..0xA000 => self.v_ram[0][(addr - 0x8000) as usize], //  VRAM
			0xA000..0xC000 => self.cartridge_state.read(addr), //  Cartrage RAM
			0xC000..0xD000 => self.w_ram[0][(addr - 0xC000) as usize], // Internal RAM
			0xD000..0xE000 => self.w_ram[1][(addr - 0xD000) as usize], // Switchable RAM in CGB mode
			0xE000..0xFE00 => self.w_ram[0][(addr - 0xE000) as usize], // Mirror, should not be used
			0xFE00..0xFEA0 => self.oam[(addr - 0xFE00) as usize], // Object Attribute Map
			0xFEA0..0xFF00 => 0x0,                             // Unusable
			0xFF00..0xFF80 => {
				logger::error(format!("TODO Addr: {:X}", addr));
				0
			} // IO Registers
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
