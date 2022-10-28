use super::cartridge::CartridgeState;
use super::cpu::registers::CPURegister16;
use super::cpu::values::ValueRefU16;
use super::cpu::{CPUState, CPU};
use super::memory_mapper::MemoryMapper;
use super::ppu::PPUState;
use crate::app::components::logger;

pub struct EmulatorState {
	pub ram_bank: u8,
	pub cgb: bool,
	pub cpu_state: CPUState,
	pub ppu_state: PPUState,
	pub cartridge_state: Option<CartridgeState>,
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
			ppu_state: PPUState::default(),
			cartridge_state: None,
			ram_bank: 0,
			cgb: false,
			interupt_register: 0,
			v_ram: [[0; 0x2000]; 2],
			w_ram: [[0; 0x1000]; 8],
			oam: [0; 0xA0],
			hram: [0; 0x80],
		}
	}
}

impl<'a> EmulatorState {
	pub fn step(&mut self) {}

	pub fn init(&mut self) {
		self.cpu_state.registers.pc = 0x100;
		self.cpu_state.registers.sp = 0xFFFE;

		self.write_16(ValueRefU16::Reg(CPURegister16::AF), 0x01B0);
		self.write_16(ValueRefU16::Reg(CPURegister16::BC), 0x0013);
		self.write_16(ValueRefU16::Reg(CPURegister16::DE), 0x00D8);
		self.write_16(ValueRefU16::Reg(CPURegister16::HL), 0x014D);

		{
			self.write(0xFF10, 0x80);
			self.write(0xFF11, 0xBF);
			self.write(0xFF12, 0xF3);
			self.write(0xFF14, 0xBF);
			self.write(0xFF16, 0x3F);
			self.write(0xFF19, 0xBF);
			self.write(0xFF1A, 0x7F);
			self.write(0xFF1B, 0xFF);
			self.write(0xFF1C, 0x9F);
			self.write(0xFF1E, 0xBF);
			self.write(0xFF20, 0xFF);
			self.write(0xFF23, 0xBF);
			self.write(0xFF24, 0x77);
			self.write(0xFF25, 0xF3);
			self.write(0xFF26, 0xF1);
			self.write(0xFF40, 0x91);
			self.write(0xFF47, 0xFC);
			self.write(0xFF48, 0xFF);
			self.write(0xFF49, 0xFF);
		}
	}

	pub fn load_rom(&mut self, rom: &Vec<u8>) {
		let mut new_rom = rom.clone();

		if new_rom.len() < 0x10000 {
			new_rom.resize(0x10000, 0);
		}

		if let Ok(state) = CartridgeState::from_raw_rom(new_rom) {
			logger::info("Loaded Rom");
			logger::info(format!("{:?}", state.info));
			self.cartridge_state.insert(state);
		} else {
			logger::error("Rom Loading Failed")
		}
	}
}

impl MemoryMapper for EmulatorState {
	fn read(&self, addr: u16) -> u8 {
		match addr {
			0x0000..0x8000 => {
				if let Some(rom) = &self.cartridge_state {
					rom.read(addr)
				} else {
					0
				}
			} // Cartridge Rom
			0x8000..0xA000 => self.v_ram[0][(addr - 0x8000) as usize], //  VRAM
			0xA000..0xC000 => {
				if let Some(rom) = &self.cartridge_state {
					rom.read(addr)
				} else {
					0
				}
			} //  Cartrage RAM
			0xC000..0xD000 => self.w_ram[0][(addr - 0xC000) as usize], // Internal RAM
			0xD000..0xE000 => self.w_ram[1][(addr - 0xD000) as usize], // Switchable RAM in CGB mode
			0xE000..0xFE00 => self.w_ram[0][(addr - 0xE000) as usize], // Mirror, should not be used
			0xFE00..0xFEA0 => self.oam[(addr - 0xFE00) as usize],      // Object Attribute Map
			0xFEA0..0xFF00 => 0x0,                                     // Unusable
			0xFF00..0xFF80 => {
				logger::error(format!("TODO Addr: {:X}", addr));
				0
			} // IO Registers
			0xFF80..0xFFFF => self.hram[(addr - 0xFF80) as usize],     // HRAM
			0xFFFF => todo!(),                                         // Interupt enable
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
