mod color_ram;
pub mod dmg_palette;
mod lcdc;
pub mod renderer;
mod sprite;
mod stat;
pub mod tile_data;

use crate::{
	lcd::GameboyLCD,
	ppu::renderer::PixelFIFO,
	util::{bits::BIT_7, BigArray},
};
use sm83::Interrupt;
use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use self::{
	color_ram::ColorRamController, dmg_palette::DMGPalette, lcdc::Lcdc, renderer::Pixel,
	sprite::Sprite, stat::Stat,
};

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub enum FetcherMode {
	#[default]
	Background,
	Window,
}

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub enum GBMode {
	#[default]
	CGB,
	DMG,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
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

impl From<u8> for VRAMBank {
	fn from(value: u8) -> Self {
		match value & 1 {
			0 => VRAMBank::Bank0,
			1 => VRAMBank::Bank1,
			_ => unreachable!(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Registers {
	pub scy: u8,
	pub scx: u8,
	pub lyc: u8,
	pub bgp: u8,
	pub obp0: u8,
	pub obp1: u8,
	pub wy: u8,
	pub wx: u8,
	pub ly: u8,
	pub stat: Stat,
	pub lcdc: Lcdc,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PPU {
	pub gb_mode: GBMode,
	pub cycle: u64,
	ran_cycles: u64,
	last_frame: u64,
	scanline_cycle_start: u64,

	#[serde(with = "BigArray")]
	pub v_ram_bank_0: [u8; 0x2000],

	/// Only in Gameboy Color mode
	#[serde(with = "BigArray")]
	pub v_ram_bank_1: [u8; 0x2000],

	#[serde(with = "BigArray")]
	pub oam: [u8; 0xA0],

	#[serde(skip)]
	pub lcd: GameboyLCD,

	pub registers: Registers,

	pub bg_color: ColorRamController,
	pub obj_color: ColorRamController,

	pub frame: u64,
	pub dmg_pallette: DMGPalette,

	mode: PPUMode,

	stat_irq: bool,
	fetcher_mode: FetcherMode,
	current_pixel: u8,
	window_line: u8,

	sprites: Vec<Sprite>,
	current_tile: u8,

	fifo_pixel: u8,
	fifo_bg: VecDeque<Pixel>,
	fifo_obj: VecDeque<Pixel>,
}

impl PPU {
	pub fn write_lcdc(&mut self, value: u8, interrupt_register: &mut u8) {
		if value & BIT_7 == 0 && self.is_enabled() {
			self.set_ly(0, interrupt_register);
		}

		if value & BIT_7 != 0 && !self.is_enabled() {
			self.current_pixel = 0;
			self.set_mode(PPUMode::HBlank, interrupt_register);
		}

		self.registers.lcdc.write(value);
	}

	pub fn read_lcdc(&self) -> u8 {
		self.registers.lcdc.read()
	}

	pub fn update_lyc(&mut self, interrupt_register: &mut u8) {
		self.registers
			.stat
			.set_lyc_eq_ly(self.registers.ly == self.registers.lyc);

		self.update_stat_irq(interrupt_register)
	}

	pub fn update_stat_irq(&mut self, interrupt_register: &mut u8) {
		let mode_int = self.registers.stat.int_enable(self.mode);
		let mode_int =
			mode_int || (self.registers.stat.int_enable(PPUMode::OamScan) && self.get_ly() == 144);

		let lyc_int = self.registers.stat.lyc_eq_ly() && self.registers.stat.lyc_eq_ly_ie();
		let new_value = mode_int || lyc_int;

		let rising_edge = !self.stat_irq && new_value;
		self.stat_irq = new_value;

		if rising_edge {
			*interrupt_register |= Interrupt::LcdStat.flag_bit();
		}
	}

	pub fn mode(&self) -> PPUMode {
		self.mode
	}

	pub fn get_ly(&self) -> u8 {
		self.registers.ly
	}

	pub fn is_enabled(&self) -> bool {
		self.registers.lcdc.display_enabled()
	}

	pub fn set_lyc(&mut self, lyc: u8, interrupt_register: &mut u8) {
		self.registers.lyc = lyc;
	}

	pub fn write_stat(&mut self, value: u8, _interrupt_register: &mut u8) {
		self.registers.stat.write(value);
	}

	pub fn read_stat(&self) -> u8 {
		if self.is_enabled() {
			self.registers.stat.read(self.is_enabled()) | self.mode as u8
		} else {
			self.registers.stat.read(self.is_enabled())
		}
	}

	pub fn set_ly(&mut self, value: u8, interrupt_register: &mut u8) {
		self.registers.ly = value;
	}

	pub fn set_mode(&mut self, mode: PPUMode, interrupt_register: &mut u8) -> Option<PPUMode> {
		self.mode = mode;

		// Don't trigger any interrupts or HDMA transfers if the screen is disabled
		if !self.is_enabled() {
			return None;
		}

		self.update_stat_irq(interrupt_register);
		if let PPUMode::VBlank = mode {
			*interrupt_register |= Interrupt::VBlank.flag_bit();
		}

		Some(mode)
	}

	pub fn step(&mut self, interrupt_register: &mut u8) -> Option<PPUMode> {
		const SCANLINE_CYCLES: u64 = 455;
		const OAM_SCAN_CYCLES: u64 = 79;

		if !self.is_enabled() {
			return None;
		}

		self.update_lyc(interrupt_register);

		self.ran_cycles += 1;
		if self.cycle > 0 {
			self.cycle -= 1;
			return None;
		}

		match self.mode {
			PPUMode::HBlank => {
				self.set_ly(self.get_ly() + 1, interrupt_register);
				if self.get_ly() < 144 {
					self.cycle += OAM_SCAN_CYCLES;
					self.scanline_cycle_start = self.ran_cycles;
					self.set_mode(PPUMode::OamScan, interrupt_register)
				} else {
					self.cycle += SCANLINE_CYCLES;
					self.window_line = 255;
					self.set_mode(PPUMode::VBlank, interrupt_register)
				}
			}
			PPUMode::VBlank => {
				if self.get_ly() < 153 {
					self.cycle += SCANLINE_CYCLES;
					self.set_ly(self.get_ly() + 1, interrupt_register);
					None
				} else {
					self.set_ly(0, interrupt_register);
					self.cycle += OAM_SCAN_CYCLES;

					self.frame += 1;
					self.lcd.swap_buffers();
					self.last_frame = self.ran_cycles;
					self.scanline_cycle_start = self.ran_cycles;
					self.set_mode(PPUMode::OamScan, interrupt_register)
				}
			}
			PPUMode::OamScan => {
				self.cycle += 11;
				self.start_scanline();
				self.set_mode(PPUMode::Draw, interrupt_register)
			}
			PPUMode::Draw => {
				self.step_fifo();
				if self.current_pixel == 160 {
					// HBlank duration varies based on how long OAM and Draw modes took
					let ran_cycles = self.ran_cycles - self.scanline_cycle_start;
					let remaining = SCANLINE_CYCLES - ran_cycles;
					self.cycle += remaining;
					self.set_mode(PPUMode::HBlank, interrupt_register)
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
			gb_mode: Default::default(),
			stat_irq: false,
			last_frame: 0,
			ran_cycles: 0,
			window_line: 0xFF,
			v_ram_bank_0: [0; 0x2000],
			v_ram_bank_1: [0; 0x2000],
			oam: [0; 0xA0],
			cycle: 0,
			mode: PPUMode::OamScan,
			lcd: Default::default(),
			registers: Registers::default(),
			bg_color: Default::default(),
			obj_color: Default::default(),
			frame: 0,
			fetcher_mode: FetcherMode::Background,
			current_pixel: 0,
			sprites: vec![],
			current_tile: 0,
			fifo_pixel: 0,
			fifo_bg: VecDeque::with_capacity(16),
			fifo_obj: VecDeque::with_capacity(16),
			dmg_pallette: Default::default(),
			scanline_cycle_start: 0,
		}
	}
}
