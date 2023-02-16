mod color_ram;
mod renderer;
mod sprite;
mod tile_data;

use std::collections::VecDeque;

use crate::{
	flags::{LCDFlags, STATFlags},
	lcd::GameboyLCD,
	ppu::renderer::PixelFIFO,
	util::BigArray,
};

use serde::{Deserialize, Serialize};
use sm83::flags::interrupt::{LCD_STAT, V_BLANK};

use self::{color_ram::ColorRamController, renderer::Pixel, sprite::Sprite};

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
	pub lcd: Option<GameboyLCD>,

	pub scy: u8,
	pub scx: u8,
	pub lyc: u8,
	pub bgp: u8,
	pub obp0: u8,
	pub obp1: u8,
	pub wy: u8,
	pub wx: u8,
	pub stat: STATFlags,

	pub bg_color: ColorRamController,
	pub obj_color: ColorRamController,

	pub frame: u64,

	fetcher_mode: FetcherMode,
	current_pixel: u8,
	window_line: u8,
	lcdc: LCDFlags,
	ly: u8,

	sprites: Vec<Sprite>,
	current_tile: u8,

	fifo_pixel: u8,
	fifo_bg: VecDeque<Pixel>,
	fifo_obj: VecDeque<Pixel>,
}

impl PPU {
	fn request_v_blank(&mut self) {
		self.interrupt_requests |= V_BLANK;
	}

	fn request_stat(&mut self) {
		self.interrupt_requests |= LCD_STAT;
	}

	pub fn write_lcdc(&mut self, value: u8) {
		let value = LCDFlags::from_bits_truncate(value);

		if !value.contains(LCDFlags::DISPLAY_ENABLE) {
			self.disable_display();
		}
		self.lcdc = value;
		self.update_lyc()
	}

	pub fn read_lcdc(&self) -> u8 {
		self.lcdc.bits()
	}

	pub fn update_lyc(&mut self) {
		if self.is_enabled() {
			let last = self.stat.contains(STATFlags::LYC_EQ_LY);

			self.stat.set(STATFlags::LYC_EQ_LY, self.ly == self.lyc);

			if self.ly == self.lyc && !last && self.stat.contains(STATFlags::LYC_EQ_LY_IE) {
				self.request_stat();
			}
		}
	}

	pub fn get_mode(&self) -> PPUMode {
		let stat = self.stat & STATFlags::PPU_MODE;

		match stat.bits() {
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
		self.lcdc.contains(LCDFlags::DISPLAY_ENABLE)
	}

	pub fn set_lyc(&mut self, lyc: u8) {
		self.lyc = lyc;
		self.update_lyc();
	}

	pub fn write_stat(&mut self, value: u8) {
		let value = STATFlags::from_bits_truncate(value) & !STATFlags::READ_ONLY;
		self.stat.remove(!STATFlags::READ_ONLY);
		self.stat |= value;
	}

	pub fn set_ly(&mut self, value: u8) {
		self.ly = value;
		self.update_lyc();
	}

	pub fn set_mode(&mut self, mode: PPUMode) {
		self.stat.remove(STATFlags::PPU_MODE);
		self.stat.insert(STATFlags::from_bits_truncate(mode as u8));

		if !self.is_enabled() {
			// Don't trigger any interrupts if the screen is disabled
			return;
		}

		// Do Interrupts
		use PPUMode::*;
		if match mode {
			HBlank => self.stat.contains(STATFlags::H_BLANK_IE),
			VBlank => self.stat.contains(STATFlags::V_BLANK_IE),
			OamScan => self.stat.contains(STATFlags::OAM_IE),
			Draw => false,
		} {
			self.request_stat()
		}

		if matches!(mode, VBlank) {
			self.request_v_blank();
		}
	}

	fn disable_display(&mut self) {
		self.set_ly(0);
		self.current_pixel = 0;
		self.set_mode(PPUMode::HBlank);
	}

	pub fn step_ppu_cycles(&mut self, cycles: u64) {
		if self.cycle > cycles {
			self.cycle -= cycles
		} else {
			for _ in 0..cycles {
				self.step_ppu();
			}
		}
	}

	pub fn step_ppu(&mut self) -> Option<PPUMode> {
		if !self.is_enabled() {
			return None;
		}

		if self.cycle != 0 {
			self.cycle -= 1;
			return None;
		}

		use PPUMode::*;
		match self.get_mode() {
			HBlank => {
				self.set_ly(self.get_ly() + 1);
				if self.get_ly() < 144 {
					self.cycle += 80;
					self.set_mode(OamScan);
					Some(OamScan)
				} else {
					self.cycle += 456;
					self.window_line = 255;

					if let Some(lcd) = &mut self.lcd {
						self.frame += 1;
						lcd.swap_buffers();
					}

					self.set_mode(VBlank);
					Some(VBlank)
				}
			}
			VBlank => {
				if self.get_ly() < 153 {
					self.cycle += 456;
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
				self.cycle += 12;
				self.start_scanline();
				self.set_mode(Draw);
				Some(Draw)
			}
			Draw => {
				self.step_fifo();
				if self.current_pixel == 160 {
					self.current_pixel = 0;
					self.current_tile = 0;
					self.cycle += 204;
					self.set_mode(HBlank);
					Some(HBlank)
				} else {
					None
				}
			}
		}
	}
}

impl Default for PPU {
	fn default() -> Self {
		Self {
			window_line: 0xFF,
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
			stat: STATFlags::empty(),
			bg_color: Default::default(),
			obj_color: Default::default(),
			frame: 0,
			fetcher_mode: FetcherMode::Background,
			current_pixel: 0,
			lcdc: LCDFlags::empty(),
			ly: 0,
			sprites: vec![],
			current_tile: 0,
			fifo_pixel: 0,
			fifo_bg: Default::default(),
			fifo_obj: Default::default(),
		}
	}
}
