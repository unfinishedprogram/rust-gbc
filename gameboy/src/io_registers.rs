use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use crate::{
	flags::{INTERRUPT_REQUEST, INT_SERIAL},
	memory_mapper::Source,
	memory_mapper::SourcedMemoryMapper,
	state::GameboyMode,
	util::{bits::*, BigArray},
	work_ram::BankedWorkRam,
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

pub const HDMA1: u16 = 0xFF51;
pub const HDMA2: u16 = 0xFF52;
pub const HDMA3: u16 = 0xFF53;
pub const HDMA4: u16 = 0xFF54;
pub const HDMA5: u16 = 0xFF55;

#[derive(Clone, Serialize, Deserialize)]
pub struct IORegisterState {
	#[serde(with = "BigArray")]
	values: [u8; 0x80],
}

impl Default for IORegisterState {
	fn default() -> Self {
		Self { values: [0; 0x80] }
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
			// PPU
			LCDC => self.ppu.read_lcdc(),

			SCY => self.ppu.scy,
			SCX => self.ppu.scx,
			LYC => self.ppu.lyc,
			BGP => self.ppu.bgp,
			OBP0 => self.ppu.obp0,
			OBP1 => self.ppu.obp1,
			WY => self.ppu.wy,
			WX => self.ppu.wx,
			LY => self.ppu.get_ly(),
			STAT => self.ppu.stat.bits(),

			// Gameboy Color only pallettes
			0xFF68..=0xFF6B => {
				if let GameboyMode::GBC(_) = &self.mode {
					match addr {
						BGPI => self.ppu.bg_color.read_spec(),
						BGPD => self.ppu.bg_color.read_data(),
						OBPI => self.ppu.obj_color.read_spec(),
						OBPD => self.ppu.obj_color.read_data(),
						_ => unreachable!("{addr}"),
					}
				} else {
					0xFF
				}
			}

			// Timer
			DIV => self.timer.get_div(),
			TAC => self.timer.get_tac(),
			TIMA => self.timer.get_tima(),
			TMA => self.timer.get_tma(),

			//HDMA
			HDMA5 => self.dma_controller.read_hdma5(),

			SVBK => {
				if let GameboyMode::GBC(_) = &self.mode {
					self.w_ram.get_bank_number()
				} else {
					0xFF
				}
			}
			VBK => {
				if let GameboyMode::GBC(_) = &self.mode {
					self.w_ram.get_bank_number()
				} else {
					0xFF
				}
			}
			KEY1 => {
				if let GameboyMode::GBC(state) = &self.mode {
					state.read_key1()
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

			// Interrupt requests
			IF => {
				self.io_register_state[IF]
					| self.ppu.interrupt_requests
					| self.timer.interrupt_requests
					| 0xE0
			}
			IE => self.io_register_state[IE] | 0xE0,
			_ => self.io_register_state[addr],
		}
	}

	fn write_io(&mut self, addr: u16, value: u8) {
		match addr {
			// PPU
			LCDC => self.ppu.write_lcdc(value),

			SCY => self.ppu.scy = value,
			SCX => self.ppu.scx = value,
			LYC => self.ppu.set_lyc(value),
			BGP => self.ppu.bgp = value,
			OBP0 => self.ppu.obp0 = value,
			OBP1 => self.ppu.obp1 = value,
			WY => self.ppu.wy = value,
			WX => self.ppu.wx = value,
			STAT => self.ppu.write_stat(value),
			HDMA1 => self.dma_controller.write_source_high(value),
			HDMA2 => self.dma_controller.write_source_low(value),
			HDMA3 => self.dma_controller.write_destination_high(value),
			HDMA4 => self.dma_controller.write_destination_low(value),
			HDMA5 => {
				if let Some(request) = self.dma_controller.write_hdma5(value) {
					self.handle_transfer(request)
				}
			}

			// Timer
			DIV => self.timer.set_div(value),
			TAC => self.timer.set_tac(value),
			TIMA => self.timer.set_tima(value),
			TMA => self.timer.set_tma(value),

			// Gameboy Color only pallettes
			0xFF68..=0xFF6B => {
				if let GameboyMode::GBC(_) = &mut self.mode {
					match addr {
						BGPI => self.ppu.bg_color.write_spec(value),
						BGPD => self.ppu.bg_color.write_data(value),
						OBPI => self.ppu.obj_color.write_spec(value),
						OBPD => self.ppu.obj_color.write_data(value),
						_ => unreachable!("{addr}"),
					}
				}
			}
			SVBK => {
				self.w_ram.set_bank_number(value);
			}
			VBK => {
				if let GameboyMode::GBC(state) = &mut self.mode {
					state.set_vram_bank(value);
				};
			}
			KEY1 => {
				if let GameboyMode::GBC(state) = &mut self.mode {
					state.write_key1(value);
				};
			}

			DISABLE_BOOT => {
				self.booting = false;
			}
			SB => self.io_register_state[SB] = value,
			JOYP => {
				self.io_register_state[JOYP] = value & 0b00110000;
			}
			SC => {
				if value == 0x81 {
					self.io_register_state[SC] = 0x01;
					self.io_register_state[SB] = 0xFF;
					self.request_interrupt(INT_SERIAL);
				}
			}

			IF => {
				self.io_register_state[INTERRUPT_REQUEST] = value & 0b00011111;
				self.ppu.interrupt_requests = value & 0b00011111;
				self.timer.interrupt_requests = value & 0b00011111;
			}

			IE => self.io_register_state[IE] = value & 0b00011111,
			DMA => {
				self.io_register_state[DMA] = value;

				// Indexing into HRAM should use work-ram instead.
				let value = if value > 0xDF { value - 0x20 } else { value };

				let mut oam_data = vec![0; 0xA0];
				let real_addr = (value as u16) << 8;

				(0..0xA0).for_each(|i| {
					oam_data[i] = self.read_from(real_addr + i as u16, Source::Raw);
				});

				self.oam_dma.start_oam_dma(oam_data);
			}

			_ => self.io_register_state[addr] = value,
		}
	}
}
