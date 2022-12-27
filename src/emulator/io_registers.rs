use std::ops::{Index, IndexMut};

use log::error;

use crate::{
	emulator::{memory_mapper::MemoryMapper, ppu::PPU},
	util::bits::bit,
};

use super::EmulatorState;

// Timers
pub const DIV: u16 = 0xFF04;
pub const TIMA: u16 = 0xFF05;
pub const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;

// Sound
pub const NR10: u16 = 0xFF10;
pub const NR11: u16 = 0xFF11;
pub const NR12: u16 = 0xFF12;
pub const NR14: u16 = 0xFF14;
pub const NR21: u16 = 0xFF16;
pub const NR22: u16 = 0xFF17;
pub const NR24: u16 = 0xFF19;
pub const NR30: u16 = 0xFF1A;
pub const NR31: u16 = 0xFF1B;
pub const NR32: u16 = 0xFF1C;
pub const NR33: u16 = 0xFF1E;
pub const NR41: u16 = 0xFF20;
pub const NR42: u16 = 0xFF21;
pub const NR43: u16 = 0xFF22;
pub const NR44: u16 = 0xFF23;
pub const NR50: u16 = 0xFF24;
pub const NR51: u16 = 0xFF25;
pub const NR52: u16 = 0xFF26;

// PPU
pub const LCDC: u16 = 0xFF40;
pub const STAT: u16 = 0xFF41;
pub const SCY: u16 = 0xFF42;
pub const SCX: u16 = 0xFF43;
pub const LY: u16 = 0xFF44;
pub const LYC: u16 = 0xFF45;
pub const DMA: u16 = 0xFF46;

pub const BGP: u16 = 0xFF47; // Background Pallete data non CGB mode only
pub const OBP0: u16 = 0xFF48; // Object Palette 0 Data data non CGB mode only
pub const OBP1: u16 = 0xFF49; // Object Palette 1 Data data non CGB mode only

pub const WY: u16 = 0xFF4A;
pub const WX: u16 = 0xFF4B;

// Serial Transfer
pub const SB: u16 = 0xFF01;
pub const SC: u16 = 0xFF02;
pub const IF: u16 = 0xFF0F;
pub const IE: u16 = 0xFFFF;
pub const JOYP: u16 = 0xFF00;

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
		match addr {
			JOYP => {
				if self.io_register_state[JOYP] & bit(4) == bit(4) {
					self.raw_joyp_input & 0b1111
				} else if self.io_register_state[addr] & bit(5) == bit(5) {
					(self.raw_joyp_input >> 4) & 0b1111
				} else {
					0b1111
				}
			}
			TAC => self.io_register_state[addr] | 0xF8,
			_ => self.io_register_state[addr],
		}
	}

	fn write_io(&mut self, addr: u16, value: u8) {
		match addr {
			DIV => self.io_register_state[DIV] = 0,
			SB => self.io_register_state[0xFF01] = value,
			JOYP => {
				self.io_register_state[JOYP] = value & 0b00110000;
			}
			SC => {
				if value == 0x81 {
					self.serial_output.push(self.io_register_state[0xFF01]);
				}
			}
			LCDC => {
				if value & 0b10000000 == 0 || self.io_register_state[LCDC] & 0b10000000 == 0 {
					self.set_ly(0);
					self.ppu_state.pause();
				}
				self.io_register_state[LCDC] = value;
			}
			IF => self.io_register_state[IF] = value & 0b00011111,
			IE => self.io_register_state[IE] = value & 0b00011111,
			DMA => {
				let real_addr = (value as u16) * 0x100;
				for i in 0..0xA0u16 {
					self.oam[i as usize] = self.read(real_addr + i);
				}
			}
			_ => self.io_register_state[addr] = value,
		}
	}
}
