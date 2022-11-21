use log::{error, info, warn};

use super::cartridge::CartridgeState;
use super::cpu::registers::CPURegister16;
use super::cpu::values::ValueRefU16;
use super::cpu::{CPUState, CPU};
use super::io_registers::{IORegisterState, IORegisters};
use super::lcd::LCDDisplay;
use super::memory_mapper::MemoryMapper;
use super::ppu::{PPUMode, PPUState, PPU};

trait LCDDisplayWithCopy: LCDDisplay + Copy {}

#[derive(Clone)]
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
	pub io_register_state: IORegisterState,
	pub run: bool,
	pub cycle: u64,
	pub serial_output: Vec<u8>,
}

impl PPUState {
	pub fn pause(&mut self) {
		if !self.paused {
			self.paused = true;
			self.cycle += 190000 - (5080 - 6);
		}
	}
}

impl Default for EmulatorState {
	fn default() -> Self {
		Self {
			run: false,
			cpu_state: CPUState::default(),
			ppu_state: PPUState::default(),
			io_register_state: IORegisterState::default(),
			cartridge_state: None,
			ram_bank: 0,
			cgb: false,
			interupt_register: 0,
			v_ram: [[0; 0x2000]; 2],
			w_ram: [[0; 0x1000]; 8],
			oam: [0; 0xA0],
			hram: [0; 0x80],
			cycle: 0,
			serial_output: vec![],
		}
	}
}

impl EmulatorState {
	pub fn step(&mut self, lcd: &mut dyn LCDDisplay) {
		while self.cycle >= self.ppu_state.cycle >> 4 {
			self.step_ppu(lcd);
		}

		CPU::step(self);

		while self.cycle >= self.ppu_state.cycle >> 4 {
			self.step_ppu(lcd);
		}
	}

	pub fn init(mut self) -> Self {
		self.write_16(ValueRefU16::Reg(CPURegister16::AF), 0x01B0);
		self.write_16(ValueRefU16::Reg(CPURegister16::BC), 0x0013);
		self.write_16(ValueRefU16::Reg(CPURegister16::DE), 0x00D8);
		self.write_16(ValueRefU16::Reg(CPURegister16::HL), 0x014D);

		self.write_16(ValueRefU16::Reg(CPURegister16::SP), 0xFFFE);
		self.write_16(ValueRefU16::Reg(CPURegister16::PC), 0x0100);
		self.set_mode(PPUMode::OamScan);
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
		self.ppu_state.cycle = 512;

		self
	}

	pub fn load_rom(&mut self, rom: &[u8]) {
		let mut new_rom = rom.to_owned();

		if new_rom.len() < 0x10000 {
			new_rom.resize(0x10000, 0);
		}

		if let Ok(state) = CartridgeState::from_raw_rom(new_rom) {
			info!("Loaded Rom");
			info!("{:?}", state.info);

			_ = self.cartridge_state.insert(state);
		} else {
			error!("Rom Loading Failed")
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
			0xE000..0xFE00 => self.read(addr - 0x2000),                // Mirror, should not be used
			0xFE00..0xFEA0 => self.oam[(addr - 0xFE00) as usize],      // Object Attribute Map
			0xFEA0..0xFF00 => 0x0,                                     // Unusable
			0xFF00..0xFF80 => self.read_io(addr),                      // IO Registers
			0xFF80..0xFFFF => self.hram[(addr - 0xFF80) as usize],     // HRAM
			0xFFFF => 0,                                               // Interupt enable
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
				}
			} // Cartrage RAM
			0xC000..0xD000 => self.w_ram[0][(addr - 0xC000) as usize] = value, // Internal RAM
			0xD000..0xE000 => self.w_ram[1][(addr - 0xD000) as usize] = value, // Switchable RAM in CGB mode
			0xE000..0xFE00 => self.write(addr - 0x2000, value),                // Mirror, should not be used
			0xFE00..0xFEA0 => self.oam[(addr - 0xFE00) as usize] = value,      // Object Attribute Map
			0xFEA0..0xFF00 => warn!("write to unusable memory"),               // Unusable
			0xFF00..0xFF80 => self.write_io(addr, value),                      // IO Registers
			0xFF80..0xFFFF => self.hram[(addr - 0xFF80) as usize] = value,     // HRAM
			0xFFFF => {}                                                       // Interupt enable
		}
	}
}
