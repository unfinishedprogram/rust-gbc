mod color_ram;
mod renderer;
mod sprite;
mod tile_data;

use std::collections::VecDeque;

use crate::{
	flags::{LCD_DISPLAY_ENABLE, STAT_H_BLANK_IE},
	lcd::LCD,
	ppu::renderer::PixelFIFO,
	util::BigArray,
};

use serde::{Deserialize, Serialize};

use self::{color_ram::ColorRamController, renderer::Pixel, sprite::Sprite};

use super::flags::{
	INT_LCD_STAT, INT_V_BLANK, STAT_LYC_EQ_LY, STAT_LYC_EQ_LY_IE, STAT_OAM_IE, STAT_V_BLANK_IE,
};

#[derive(Clone, Serialize, Deserialize, Default)]
enum FetcherMode {
	#[default]
	Background,
	Window,
}

#[derive(Debug, Clone, Copy)]
pub enum PPUMode {
	HBlank = 0,
	VBlank = 1,
	OamScan = 2,
	Draw = 3,
}

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub enum VRAMBank {
	#[default]
	Bank0,
	Bank1,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PPU {
	pub cycle: u64,

	#[serde(with = "BigArray")]
	pub v_ram_bank_0: [u8; 0x2000],

	/// Only in Gameboy Color mode
	#[serde(with = "BigArray")]
	pub v_ram_bank_1: [u8; 0x2000],

	#[serde(with = "BigArray")]
	pub oam: [u8; 0xA0],
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

	pub frame: u64,

	fetcher_mode: FetcherMode,
	current_pixel: u8,
	window_line: u8,
	enabled: bool,
	lcdc: u8,
	ly: u8,

	sprites: Vec<Sprite>,
	current_tile: u8,

	fifo_pixel: u8,
	fifo_bg: VecDeque<Pixel>,
	fifo_obj: VecDeque<Pixel>,
	pub h_blank_hit: bool,
}

impl PPU {
	pub fn new() -> Self {
		Self {
			h_blank_hit: false,
			window_line: 0xFF,
			enabled: true,
			v_ram_bank_0: [0; 0x2000],
			v_ram_bank_1: [0; 0x2000],
			oam: [0; 0xA0],
			cycle: 0,
			interrupt_requests: 0,
			lcd: None,
			scy: 0,
			scx: 0,
			lyc: 0,
			bgp: 0,
			obp0: 0,
			obp1: 0,
			wy: 0,
			wx: 0,
			stat: 0,
			bg_color: Default::default(),
			obj_color: Default::default(),
			frame: 0,
			fetcher_mode: FetcherMode::Background,
			current_pixel: 0,
			lcdc: 0,
			ly: 0,
			sprites: vec![],
			current_tile: 0,
			fifo_pixel: 0,
			fifo_bg: Default::default(),
			fifo_obj: Default::default(),
		}
	}

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

	pub fn is_enabled(&self) -> bool {
		self.enabled
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

		if !self.enabled {
			// Don't trigger any interrupts if the screen is disabled
			return;
		}

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

	pub fn step_ppu_cycles(&mut self, cycles: u64) {
		let to_step = if self.cycle < cycles {
			cycles - self.cycle
		} else {
			self.cycle -= cycles;
			0
		};

		for _ in 0..to_step {
			self.step_ppu();
		}
	}

	pub fn step_ppu(&mut self) -> Option<PPUMode> {
		if self.cycle != 0 {
			self.cycle -= 1;
			return None;
		}

		if !self.enabled {
			return None;
		}

		use PPUMode::*;
		match self.get_mode() {
			HBlank => {
				self.set_ly(self.get_ly() + 1);
				if self.get_ly() <= 143 {
					self.cycle += 80;
					self.set_mode(OamScan);
					return Some(OamScan);
				} else {
					self.cycle += 458;
					self.window_line = 255;

					if let Some(lcd) = &mut self.lcd {
						self.frame += 1;
						lcd.swap_buffers();
					}

					self.set_mode(VBlank);
					return Some(VBlank);
				}
			}
			VBlank => {
				if self.get_ly() < 153 {
					self.cycle += 458;
					self.set_ly(self.get_ly() + 1);
					None
				} else {
					self.set_ly(0);
					self.cycle += 80;
					self.set_mode(OamScan);
					Some(OamScan)
				}
			}
			OamScan => {
				// self.scanline_state = self.fetch_scanline_state();
				self.cycle += 12;
				self.start_scanline();
				self.set_mode(Draw);
				Some(Draw)
			}
			Draw => {
				self.step_fifo();
				// self.render_screen_pixel(self.current_pixel, self.ly);
				if self.current_pixel == 160 {
					self.current_pixel = 0;
					self.current_tile = 0;
					self.cycle += 204;
					self.h_blank_hit = true;
					self.set_mode(HBlank);
					Some(HBlank)
				} else {
					self.cycle += 1;
					None
				}
			}
		}
	}
}