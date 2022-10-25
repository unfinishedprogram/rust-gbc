use std::rc::{self, Rc};

use crate::app::components::logger;

use super::cartridge::CartridgeState;
use super::cpu::CPUState;

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
	pub fn load_rom(&mut self, rom: Rc<Vec<u8>>) {
		let rom_data = Rc::new(Box::new(rom));

		if let Ok(state) = CartridgeState::from_raw_rom(Rc::clone(&rom_data)) {
			self.cartridge_state = state;
			logger::info("Loaded Rom")
		} else {
			logger::error("Rom Loading Failed")
		}
	}
}
