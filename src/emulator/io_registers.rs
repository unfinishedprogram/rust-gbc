use std::ops::{Index, IndexMut};

use log::error;

use crate::{
	emulator::{memory_mapper::MemoryMapper, ppu::PPU},
	util::bits::bit,
};

use super::EmulatorState;

#[derive(Clone, Copy, Primitive)]
pub enum IORegistersAddress {
	// Timers
	DIV = 0xFF04,
	TIMA = 0xFF05,
	TMA = 0xFF06,
	TAC = 0xFF07,

	// Sound
	NR10 = 0xFF10,
	NR11 = 0xFF11,
	NR12 = 0xFF12,
	NR14 = 0xFF14,
	NR21 = 0xFF16,
	NR22 = 0xFF17,
	NR24 = 0xFF19,
	NR30 = 0xFF1A,
	NR31 = 0xFF1B,
	NR32 = 0xFF1C,
	NR33 = 0xFF1E,
	NR41 = 0xFF20,
	NR42 = 0xFF21,
	NR43 = 0xFF22,
	NR44 = 0xFF23,
	NR50 = 0xFF24,
	NR51 = 0xFF25,
	NR52 = 0xFF26,

	// PPU
	LCDC = 0xFF40,
	STAT = 0xFF41,
	SCY = 0xFF42,
	SCX = 0xFF43,
	LY = 0xFF44,
	LYC = 0xFF45,
	DMA = 0xFF46,

	BGP = 0xFF47,  // Background Pallete data non CGB mode only
	OBP0 = 0xFF48, // Object Palette 0 Data data non CGB mode only
	OBP1 = 0xFF49, // Object Palette 1 Data data non CGB mode only

	WY = 0xFF4A,
	WX = 0xFF4B,

	// Serial Transfer
	SB = 0xFF01,
	SC = 0xFF02,
	IF = 0xFF0F,
	IE = 0xFFFF,
	JOYP = 0xFF00,
}

#[derive(Clone)]
pub struct IORegisterState {
	values: [u8; 0x80],
	_other: u8,
}
impl Default for IORegisterState {
	fn default() -> Self {
		Self {
			values: [0; 0x80],
			_other: 0,
		}
	}
}

impl Index<u16> for IORegisterState {
	type Output = u8;

	fn index(&self, index: u16) -> &Self::Output {
		match index {
			0xFF00..0xFF80 => &self.values[(index - 0xFF00) as usize],
			_ => {
				error!("read from invalid IORegister: {:X}", index);
				&self._other
			}
		}
	}
}

impl IndexMut<u16> for IORegisterState {
	fn index_mut(&mut self, index: u16) -> &mut Self::Output {
		match index {
			0xFF00..0xFF80 => &mut self.values[(index - 0xFF00) as usize],
			_ => {
				error!("write to invalid IORegister: {:X}", index);
				&mut self._other
			}
		}
	}
}

pub trait IORegisters {
	fn read_io(&self, addr: u16) -> u8;
	fn write_io(&mut self, addr: u16, value: u8);
}

impl IORegisters for EmulatorState {
	fn read_io(&self, addr: u16) -> u8 {
		match IORegistersAddress::try_from(addr) {
			// All 0 is off
			Ok(IORegistersAddress::JOYP) => {
				if self.io_register_state[addr] & bit(4) == bit(4) {
					self.raw_joyp_input & 0b1111
				} else if self.io_register_state[addr] & bit(5) == bit(5) {
					(self.raw_joyp_input >> 4) & 0b1111
				} else {
					0b1111
				}
			}
			Ok(IORegistersAddress::TAC) => self.io_register_state[addr] | 0xF8,
			Err(_) => {
				error!("Unhandled Read: {:X}", addr);
				0xFF
			}
			_ => self.io_register_state[addr],
		}
	}

	fn write_io(&mut self, addr: u16, value: u8) {
		use IORegistersAddress::*;
		match IORegistersAddress::try_from(addr) {
			Ok(DIV) => self.io_register_state[addr] = 0,
			Ok(SB) => self.io_register_state[0xFF01] = value,
			Ok(JOYP) => {
				self.io_register_state[addr] = value & 0b00110000;
			}
			Ok(SC) => {
				if value == 0x81 {
					self.serial_output.push(self.io_register_state[0xFF01]);
				}
			}
			Ok(LCDC) => {
				if value & 0b10000000 == 0 || self.io_register_state[addr] & 0b10000000 == 0 {
					self.set_ly(0);
					self.ppu_state.pause();
				}
				self.io_register_state[addr] = value;
			}
			Ok(IF) => self.io_register_state[addr] = value & 0b00011111,
			Ok(IE) => self.io_register_state[addr] = value & 0b00011111,
			Ok(DMA) => {
				let real_addr = (value as u16) * 0x100;
				for i in 0..0xA0u16 {
					self.oam[i as usize] = self.read(real_addr + i);
				}
			}
			Err(_) => {
				self.run = false;
				error!("Unhandled Write: {:X}", addr);
			}
			Ok(_) => self.io_register_state[addr] = value,
		}
	}
}
