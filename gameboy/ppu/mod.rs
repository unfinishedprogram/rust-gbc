mod renderer;
mod sprite;

use crate::{flags::STAT_H_BLANK_IE, memory_mapper::SourcedMemoryMapper};
use serde::{Deserialize, Serialize};

use super::{
	flags::{
		INT_LCD_STAT, INT_V_BLANK, STAT_LYC_EQ_LY, STAT_LYC_EQ_LY_IE, STAT_OAM_IE, STAT_V_BLANK_IE,
	},
	io_registers::{LY, LYC, STAT},
	memory_mapper::Source,
	Gameboy,
};

use renderer::{Renderer, ScanlineState};

#[derive(Debug)]
pub enum PPUMode {
	HBlank = 0,
	VBlank = 1,
	OamScan = 2,
	Draw = 3,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct PPUState {
	pub cycle: u64,
	pub paused: bool,
	pub window_line: u8,
	pub current_pixel: u8,
	pub scanline_state: ScanlineState,
	enabled: bool,
}

pub trait PPU {
	fn get_mode(&self) -> PPUMode;
	fn get_ly(&self) -> u8;
	fn set_ly(&mut self, value: u8);
	fn set_mode(&mut self, mode: PPUMode);
	fn step_ppu(&mut self);
	fn disable_display(&mut self);
	fn enable_display(&mut self);
}

impl PPU for Gameboy {
	fn get_mode(&self) -> PPUMode {
		let num = self.read_from(STAT, Source::Ppu) & 0b00000011;
		match num {
			0 => PPUMode::HBlank,
			1 => PPUMode::VBlank,
			2 => PPUMode::OamScan,
			3 => PPUMode::Draw,
			_ => unreachable!(), // Since we only take the last two bits
		}
	}

	fn get_ly(&self) -> u8 {
		self.read_from(LY, Source::Ppu)
	}

	fn set_ly(&mut self, value: u8) {
		let lyc_status = self.read_from(LYC, Source::Ppu) == value;
		self.write_from(LY, value, Source::Ppu);

		if !self.ppu_state.enabled {
			return;
		}

		if lyc_status {
			self.io_register_state[STAT] |= STAT_LYC_EQ_LY;
			if self.io_register_state[STAT] & STAT_LYC_EQ_LY_IE != 0 {
				self.request_interrupt(INT_LCD_STAT);
			}
		} else {
			self.io_register_state[STAT] &= !STAT_LYC_EQ_LY
		}
	}

	fn set_mode(&mut self, mode: PPUMode) {
		let stat = self.read_from(STAT, Source::Ppu);

		// Do Interrupts
		use PPUMode::*;
		let interrupt_triggered = match mode {
			HBlank => stat & STAT_H_BLANK_IE != 0,
			VBlank => stat & STAT_V_BLANK_IE != 0,
			OamScan => stat & STAT_OAM_IE != 0,
			Draw => false,
		};

		if matches!(mode, VBlank) {
			self.request_interrupt(INT_V_BLANK);
		}

		if interrupt_triggered {
			self.request_interrupt(INT_LCD_STAT);
		}

		self.write_from(STAT, (stat & 0b11111100) | mode as u8, Source::Ppu);
	}

	fn disable_display(&mut self) {
		self.ppu_state.enabled = false;
		self.ppu_state.current_pixel = 0;
		self.set_mode(PPUMode::HBlank);
		self.set_ly(0);
	}

	fn enable_display(&mut self) {
		self.ppu_state.enabled = true;
	}

	fn step_ppu(&mut self) {
		if self.ppu_state.cycle != 0 {
			self.ppu_state.cycle -= 1;
			return;
		}

		if !self.ppu_state.enabled {
			self.ppu_state.cycle += 8;
			return;
		}

		use PPUMode::*;
		match self.get_mode() {
			HBlank => {
				self.set_ly(self.get_ly() + 1);
				if self.get_ly() <= 143 {
					self.ppu_state.cycle += 80;
					self.set_mode(OamScan);
				} else {
					self.ppu_state.cycle += 458;
					self.ppu_state.window_line = 0;
					self.request_interrupt(INT_V_BLANK);

					if let Some(lcd) = &mut self.lcd {
						lcd.swap_buffers();
					}

					self.set_mode(VBlank);
				}
			}
			VBlank => {
				if self.get_ly() < 153 {
					self.ppu_state.cycle += 458;
					self.set_ly(self.get_ly() + 1);
				} else {
					self.set_ly(0);
					self.ppu_state.cycle += 80;
					self.set_mode(OamScan);
				}
			}
			OamScan => {
				self.ppu_state.scanline_state = self.fetch_scanline_state();
				self.ppu_state.cycle += 12;
				self.set_mode(Draw);
			}
			Draw => {
				self.render_screen_pixel(self.ppu_state.current_pixel, self.get_ly());
				self.ppu_state.current_pixel += 1;
				if self.ppu_state.current_pixel == 160 {
					self.ppu_state.current_pixel = 0;
					self.ppu_state.cycle += 204;
					self.set_mode(HBlank);
				} else {
					self.ppu_state.cycle += 1;
				}
			}
		}
	}
}
