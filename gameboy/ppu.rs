mod color_ram;
mod renderer;
mod renderer_old;
mod sprite;
mod tile_data;

use std::collections::VecDeque;

use crate::{
	flags::{LCD_DISPLAY_ENABLE, STAT_H_BLANK_IE},
	lcd::LCD,
	ppu::{renderer::PixelFIFO, renderer_old::Renderer},
};
use serde::{Deserialize, Serialize};

use self::{color_ram::ColorRamController, renderer::Pixel};

use super::flags::{
	INT_LCD_STAT, INT_V_BLANK, STAT_LYC_EQ_LY, STAT_LYC_EQ_LY_IE, STAT_OAM_IE, STAT_V_BLANK_IE,
};

#[derive(Clone, Serialize, Deserialize)]
enum FetcherMode {
	Background,
	Window,
}

use renderer_old::ScanlineState;

#[derive(Debug, Clone, Copy)]
pub enum PPUMode {
	HBlank = 0,
	VBlank = 1,
	OamScan = 2,
	Draw = 3,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PPU {
	pub cycle: u64,
	pub v_ram: Vec<Vec<u8>>,
	pub oam: Vec<u8>,
	pub interrupt_requests: u8,

	#[serde(skip)]
	pub lcd: Option<LCD>,

	pub scy: u8,
	pub scx: u8,
	pub lyc: u8,
	pub bgp: u8,
	pub obp0: u8,
	pub obp1: u8,
	pub wy: u8,
	pub wx: u8,
	pub stat: u8,

	pub bg_color: ColorRamController,
	pub obj_color: ColorRamController,

	fetcher_mode: FetcherMode,
	scanline_state: ScanlineState,
	current_pixel: u8,
	window_line: u8,
	enabled: bool,
	lcdc: u8,
	ly: u8,

	current_tile: u8,

	fifo_pixel: u8,
	fifo_bg: VecDeque<Pixel>,
	fifo_obj: VecDeque<Pixel>,
}

impl Default for PPU {
	fn default() -> Self {
		Self {
			cycle: 0,
			window_line: 0,
			current_pixel: 0,
			fetcher_mode: FetcherMode::Background,
			scanline_state: Default::default(),
			interrupt_requests: 0,
			current_tile: 0,
			lcd: Default::default(),
			scy: Default::default(),
			scx: Default::default(),
			lyc: Default::default(),
			bgp: Default::default(),
			obp0: Default::default(),
			obp1: Default::default(),
			wy: Default::default(),
			wx: Default::default(),
			enabled: true,
			lcdc: Default::default(),
			stat: Default::default(),
			ly: Default::default(),
			fifo_pixel: Default::default(),
			fifo_bg: Default::default(),
			fifo_obj: Default::default(),
			bg_color: Default::default(),
			obj_color: Default::default(),

			v_ram: vec![vec![0; 0x2000]; 2],
			oam: vec![0; 0xA0],
		}
	}
}

impl PPU {
	fn request_v_blank(&mut self) {
		self.interrupt_requests |= INT_V_BLANK;
	}

	fn request_stat(&mut self) {
		self.interrupt_requests |= INT_LCD_STAT;
	}

	pub fn write_lcdc(&mut self, value: u8) {
		if value & LCD_DISPLAY_ENABLE == 0 {
			self.disable_display();
		} else {
			self.enable_display();
		}
		self.lcdc = value;
	}

	pub fn read_lcdc(&self) -> u8 {
		self.lcdc
	}

	pub fn get_mode(&self) -> PPUMode {
		let stat = self.stat & 0b00000011;
		match stat {
			0 => PPUMode::HBlank,
			1 => PPUMode::VBlank,
			2 => PPUMode::OamScan,
			3 => PPUMode::Draw,
			_ => unreachable!(),
		}
	}

	pub fn get_ly(&self) -> u8 {
		self.ly
	}

	pub fn set_ly(&mut self, value: u8) {
		self.ly = value;

		if !self.enabled {
			return;
		}

		if self.lyc == value {
			self.stat |= STAT_LYC_EQ_LY;
			if self.stat & STAT_LYC_EQ_LY_IE != 0 {
				self.request_stat();
			}
		} else {
			self.stat &= !STAT_LYC_EQ_LY
		}
	}

	pub fn set_mode(&mut self, mode: PPUMode) {
		self.stat = (self.stat & 0b11111100) | mode as u8;
		// Do Interrupts
		use PPUMode::*;
		if match mode {
			HBlank => self.stat & STAT_H_BLANK_IE != 0,
			VBlank => self.stat & STAT_V_BLANK_IE != 0,
			OamScan => self.stat & STAT_OAM_IE != 0,
			Draw => false,
		} {
			self.request_stat()
		}

		if matches!(mode, VBlank) {
			self.request_v_blank();
		}
	}

	fn disable_display(&mut self) {
		self.enabled = false;
		self.current_pixel = 0;
		self.set_mode(PPUMode::HBlank);
		self.set_ly(0);
	}

	fn enable_display(&mut self) {
		self.enabled = true;
	}

	pub fn step_ppu(&mut self) {
		if self.cycle != 0 {
			self.cycle -= 1;
			return;
		}

		if !self.enabled {
			self.cycle += 8;
			return;
		}

		use PPUMode::*;
		match self.get_mode() {
			HBlank => {
				self.set_ly(self.get_ly() + 1);
				if self.get_ly() <= 143 {
					self.cycle += 80;
					self.set_mode(OamScan);
				} else {
					self.cycle += 458;
					self.window_line = 0;

					if let Some(lcd) = &mut self.lcd {
						lcd.swap_buffers();
					}

					self.set_mode(VBlank);
				}
			}
			VBlank => {
				if self.get_ly() < 153 {
					self.cycle += 458;
					self.set_ly(self.get_ly() + 1);
				} else {
					self.set_ly(0);
					self.cycle += 80;
					self.set_mode(OamScan);
				}
			}
			OamScan => {
				self.scanline_state = self.fetch_scanline_state();
				self.cycle += 12;
				self.start_scanline();
				self.set_mode(Draw);
			}
			Draw => {
				self.step_fifo();
				// self.render_screen_pixel(self.current_pixel, self.ly);
				if self.current_pixel == 160 {
					self.current_pixel = 0;
					self.current_tile = 0;
					self.cycle += 204;
					self.set_mode(HBlank);
				} else {
					self.cycle += 1;
				}
			}
		}
	}
}
