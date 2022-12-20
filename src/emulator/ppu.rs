use crate::emulator::{
	flags::STAT_H_BLANK_IE, io_registers::IORegistersAddress, memory_mapper::MemoryMapper,
};

use super::{
	flags::{
		INT_LCD_STAT, INT_V_BLANK, STAT, STAT_LYC_EQ_LY, STAT_LYC_EQ_LY_IE, STAT_OAM_IE,
		STAT_V_BLANK_IE,
	},
	renderer::{Renderer, ScanlineState},
	EmulatorState,
};

#[derive(Debug)]
pub enum PPUMode {
	HBlank = 0,
	VBlank = 1,
	OamScan = 2,
	Draw = 3,
}

#[derive(Default, Clone)]
pub struct PPUState {
	pub cycle: u64,
	pub maxed: bool,
	pub paused: bool,
	pub window_line: u8,
	pub current_pixel: u8,
	pub scanline_state: ScanlineState,
}

pub trait PPU {
	fn get_mode(&self) -> PPUMode;
	fn get_ly(&self) -> u8;
	fn set_ly(&mut self, value: u8);
	fn set_mode(&mut self, mode: PPUMode);
	fn step_ppu(&mut self);
}

impl PPU for EmulatorState {
	fn get_mode(&self) -> PPUMode {
		let num = self.read(IORegistersAddress::STAT as u16) & 0b00000011;
		match num {
			0 => PPUMode::HBlank,
			1 => PPUMode::VBlank,
			2 => PPUMode::OamScan,
			3 => PPUMode::Draw,
			_ => unreachable!(), // Since we only take the last two bits
		}
	}

	fn get_ly(&self) -> u8 {
		self.read(IORegistersAddress::LY as u16)
	}

	fn set_ly(&mut self, value: u8) {
		let lyc_status = self.read(IORegistersAddress::LYC as u16) == value;
		self.write(IORegistersAddress::LY as u16, value);

		if lyc_status {
			self.io_register_state[STAT] |= STAT_LYC_EQ_LY
		} else {
			self.io_register_state[STAT] &= !STAT_LYC_EQ_LY
		}

		if lyc_status && self.io_register_state[STAT] & STAT_LYC_EQ_LY_IE != 0 {
			self.request_interrupt(INT_LCD_STAT);
		}
	}

	fn set_mode(&mut self, mode: PPUMode) {
		let stat = self.read(IORegistersAddress::STAT as u16);

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

		self.write(
			IORegistersAddress::STAT as u16,
			(stat & 0b11111100) | mode as u8,
		);
	}

	fn step_ppu(&mut self) {
		if self.ppu_state.cycle != 0 {
			self.ppu_state.cycle -= 1;
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
					self.ppu_state.cycle += 456;
					self.ppu_state.window_line = 0;
					self.request_interrupt(INT_V_BLANK);
					self.set_mode(VBlank);
				}
			}
			VBlank => {
				if self.get_ly() < 153 {
					self.ppu_state.cycle += 456;
					self.set_ly(self.get_ly() + 1);
				} else {
					self.set_ly(0);
					self.ppu_state.cycle += 80;
					self.set_mode(OamScan);
				}
			}
			OamScan => {
				self.ppu_state.scanline_state = self.fetch_scanline_state();
				self.ppu_state.cycle += 172;
				self.set_mode(Draw);
			}
			Draw => {
				self.render_screen_pixel(
					self.ppu_state.current_pixel,
					self.get_ly(),
					self.fetch_pixel_state(),
				);
				self.ppu_state.current_pixel += 1;
				if self.ppu_state.current_pixel == 160 {
					self.ppu_state.current_pixel = 0;
					self.ppu_state.cycle += 206;
					self.set_mode(HBlank);
				}
			}
		}
	}
}
