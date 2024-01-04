mod color_ram;
mod lcdc;
mod renderer;
mod sprite;
mod stat;
mod tile_data;

use crate::{lcd::GameboyLCD, ppu::renderer::PixelFIFO, util::BigArray};
use sm83::Interrupt;
use std::collections::VecDeque;

use log::debug;
use serde::{Deserialize, Serialize};

use self::{
	color_ram::ColorRamController, lcdc::Lcdc, renderer::Pixel, sprite::Sprite, stat::Stat,
};

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub enum FetcherMode {
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
	pub cycle: u64,
	ran_cycles: u64,
	last_frame: u64,

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
		if !self.is_enabled() {
			self.disable_display(interrupt_register);
		}
		self.registers.lcdc.write(value);
		self.update_lyc(interrupt_register)
	}

	pub fn read_lcdc(&self) -> u8 {
		self.registers.lcdc.read()
	}

	pub fn update_lyc(&mut self, interrupt_register: &mut u8) {
		if !self.is_enabled() {
			return;
		}

		self.registers
			.stat
			.set_lyc_eq_ly(self.registers.ly == self.registers.lyc);

		self.update_stat_irq(interrupt_register)
	}

	pub fn update_stat_irq(&mut self, interrupt_register: &mut u8) {
		let mode_int = self.registers.stat.int_enable(self.mode());
		let lyc_int = self.registers.stat.lyc_eq_ly() && self.registers.stat.lyc_eq_ly_ie();
		let new_value = mode_int || lyc_int;

		let rising_edge = !self.stat_irq && new_value;
		self.stat_irq = new_value;

		if rising_edge {
			*interrupt_register |= Interrupt::LcdStat.flag_bit();
		}
	}

	pub fn mode(&self) -> PPUMode {
		self.registers.stat.ppu_mode()
	}

	pub fn get_ly(&self) -> u8 {
		self.registers.ly
	}

	pub fn is_enabled(&self) -> bool {
		self.registers.lcdc.display_enabled()
	}

	pub fn set_lyc(&mut self, lyc: u8, interrupt_register: &mut u8) {
		self.registers.lyc = lyc;
		self.update_lyc(interrupt_register);
	}

	pub fn write_stat(&mut self, value: u8, interrupt_register: &mut u8) {
		self.registers.stat.write(value);
		self.update_stat_irq(interrupt_register)
	}

	pub fn set_ly(&mut self, value: u8, interrupt_register: &mut u8) {
		self.registers.ly = value;
		self.update_lyc(interrupt_register);
	}

	pub fn set_mode(&mut self, mode: PPUMode, interrupt_register: &mut u8) -> Option<PPUMode> {
		self.registers.stat.set_ppu_mode(mode);

		// Don't trigger any interrupts if the screen is disabled
		if !self.is_enabled() {
			return None;
		}

		self.update_stat_irq(interrupt_register);
		if let PPUMode::VBlank = mode {
			debug!("{:} VBlank", self.ran_cycles - self.last_frame);
			*interrupt_register |= Interrupt::VBlank.flag_bit();
		}

		Some(mode)
	}

	fn disable_display(&mut self, interrupt_register: &mut u8) {
		self.set_ly(0, interrupt_register);
		self.current_pixel = 0;
		self.set_mode(PPUMode::HBlank, interrupt_register);
	}

	pub fn step_ppu(&mut self, interrupt_register: &mut u8) -> Option<PPUMode> {
		if !self.is_enabled() {
			return None;
		}

		self.ran_cycles += 1;
		if self.cycle != 0 {
			self.cycle -= 1;
			return None;
		}

		match self.mode() {
			PPUMode::HBlank => {
				self.set_ly(self.get_ly() + 1, interrupt_register);
				if self.get_ly() < 144 {
					self.cycle += 79;
					self.set_mode(PPUMode::OamScan, interrupt_register)
				} else {
					self.cycle += 455;
					self.window_line = 255;
					self.set_mode(PPUMode::VBlank, interrupt_register)
				}
			}
			PPUMode::VBlank => {
				if self.get_ly() < 153 {
					self.cycle += 455;
					self.set_ly(self.get_ly() + 1, interrupt_register);
					None
				} else {
					self.set_ly(0, interrupt_register);
					self.cycle += 79;

					self.frame += 1;
					self.lcd.swap_buffers();
					debug!("Cycle:{:}", self.ran_cycles - self.last_frame);
					self.last_frame = self.ran_cycles;

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
					self.current_pixel = 0;
					self.current_tile = 0;
					self.cycle += 204;
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
			stat_irq: false,
			last_frame: 0,
			ran_cycles: 0,
			window_line: 0xFF,
			v_ram_bank_0: [0; 0x2000],
			v_ram_bank_1: [0; 0x2000],
			oam: [0; 0xA0],
			cycle: 0,
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
		}
	}
}
