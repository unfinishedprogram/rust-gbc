use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use crate::{
	flags::{INT_SERIAL, LCD_DISPLAY_ENABLE},
	memory_mapper::Source,
	memory_mapper::SourcedMemoryMapper,
	ppu::PPU,
	state::GameboyMode,
	util::bits::*,
	Gameboy,
};

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

pub const BGP: u16 = 0xFF47; // Background Pallette data non CGB mode only
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

pub const DISABLE_BOOT: u16 = 0xFF50;

/// CGB Registers

pub const VBK: u16 = 0xFF4F; //  VRAM bank
pub const RP: u16 = 0xFF56; // Infra-red comms port

/// Speed switch
///  - Bit 7: Current Speed     (0=Normal, 1=Double) (Read Only)
///  - Bit 0: Prepare Speed Switch (0=No, 1=Prepare) (Read/Write)
pub const KEY1: u16 = 0xFF4D; // Speed Switch

//https://gbdev.io/pandocs/CGB_Registers.html#ff6c--opri-cgb-mode-only-object-priority-mode
pub const OPRI: u16 = 0xFF6C; // Object priority mode

/// FF68 - BCPS/BGPI (CGB Mode only): Background color palette specification / Background palette index
pub const BGPI: u16 = 0xFF68;

/// FF69 - BCPD/BGPD (CGB Mode only): Background color palette data / Background palette data
pub const BGPD: u16 = 0xFF69;

/// FF6A - OCPS/OBPI, OCPD/OBPD (CGB Mode only): OBJ color palette specification / OBJ pallette index
pub const OBPI: u16 = 0xFF6A;

/// FF6B OBJ palette data
pub const OBPD: u16 = 0xFF6B;

pub const SVBK: u16 = 0xFF70; // WRAM bank

#[derive(Clone, Serialize, Deserialize)]
pub struct IORegisterState {
	values: Vec<u8>,
}

impl Default for IORegisterState {
	fn default() -> Self {
		Self {
			values: vec![0; 0x80],
		}
	}
}

impl Index<u16> for IORegisterState {
	type Output = u8;

	fn index(&self, index: u16) -> &Self::Output {
		&self.values[(index - 0xFF00) as usize]
	}
}

impl IndexMut<u16> for IORegisterState {
	fn index_mut(&mut self, index: u16) -> &mut Self::Output {
		&mut self.values[(index - 0xFF00) as usize]
	}
}

pub trait IORegisters {
	fn read_io(&self, addr: u16) -> u8;
	fn write_io(&mut self, addr: u16, value: u8);
}

impl IORegisters for Gameboy {
	fn read_io(&self, addr: u16) -> u8 {
		match addr {
			// Gameboy Color only pallettes
			// 0xFF68..=0xFF6B => {
			// 	if let GameboyMode::GBC(state) = &self.mode {
			// 		match addr {
			// 			BGPI => state.bg_color.read_spec(),
			// 			BGPD => state.bg_color.read_data(),
			// 			OBPI => state.obj_color.read_spec(),
			// 			OBPD => state.obj_color.read_data(),
			// 			_ => unreachable!("{addr}"),
			// 		}
			// 	} else {
			// 		0xFF
			// 	}
			// }
			SVBK => {
				if let GameboyMode::GBC(state) = &self.mode {
					state.get_wram_bank() as u8
				} else {
					0xFF
				}
			}
			VBK => {
				if let GameboyMode::GBC(state) = &self.mode {
					state.get_vram_bank() as u8
				} else {
					0xFF
				}
			}
			JOYP => {
				if self.io_register_state[JOYP] & BIT_4 == BIT_4 {
					(self.raw_joyp_input & 0b1111) | 0b11000000
				} else if self.io_register_state[addr] & BIT_5 == BIT_5 {
					((self.raw_joyp_input >> 4) & 0b1111) | 0b11000000
				} else {
					0b11001111
				}
			}
			TAC => self.io_register_state[addr] | 0xF8,
			_ => self.io_register_state[addr],
		}
	}

	fn write_io(&mut self, addr: u16, value: u8) {
		match addr {
			// Gameboy Color only pallettes
			0xFF68..=0xFF6B => {
				if let GameboyMode::GBC(state) = &mut self.mode {
					match addr {
						BGPI => state.bg_color.write_spec(value),
						BGPD => state.bg_color.write_data(value),
						OBPI => state.obj_color.write_spec(value),
						OBPD => state.obj_color.write_data(value),
						_ => unreachable!("{addr}"),
					}
				}
			}
			SVBK => {
				if let GameboyMode::GBC(state) = &mut self.mode {
					state.set_wram_bank(value);
				};
			}
			VBK => {
				if let GameboyMode::GBC(state) = &mut self.mode {
					state.set_vram_bank(value);
				};
			}

			DISABLE_BOOT => {
				self.booting = false;
			}
			DIV => {
				self.io_register_state[DIV] = 0;
				self.io_register_state[TIMA] = self.io_register_state[TMA];
				self.timer_state.timer_clock = 0;
			}
			SB => self.io_register_state[0xFF01] = value,
			JOYP => {
				self.io_register_state[JOYP] = value & 0b00110000;
			}
			SC => {
				if value == 0x81 {
					self.io_register_state[SC] = 0x01;
					self.io_register_state[0xFF01] = 0xFF;
					self.request_interrupt(INT_SERIAL);
				}
			}
			LCDC => {
				if value & LCD_DISPLAY_ENABLE == 0 {
					self.disable_display();
				} else {
					self.enable_display();
				}
				self.io_register_state[LCDC] = value;
			}
			IF => self.io_register_state[IF] = value & 0b00011111,
			IE => self.io_register_state[IE] = value & 0b00011111,
			DMA => {
				self.io_register_state[DMA] = value;

				// Indexing into HRAM should use work-ram instead.
				let value = if value > 0xDF { value - 0x20 } else { value };

				let real_addr = (value as u16) << 8;
				for i in 0..0xA0 {
					self.oam[i] = self.read_from(real_addr + i as u16, Source::Raw);
				}
				self.dma_timer += 160;
			}
			_ => self.io_register_state[addr] = value,
		}
	}
}
