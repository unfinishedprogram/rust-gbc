use crate::emulator::{io_registers::IORegistersAddress, memory_mapper::MemoryMapper};

use super::{
	flags::{
		INTERRUPT_REQUEST, INT_LCD_STAT, INT_V_BLANK, STAT, STAT_LYC_EQ_LY, STAT_LYC_EQ_LY_IE,
	},
	lcd::LCDDisplay,
	renderer::Renderer,
	EmulatorState,
};

#[derive(Debug)]
pub enum PPUMode {
	HBlank = 0,
	VBlank = 1,
	OamScan = 2,
	Draw = 3,
}

#[derive(Default, Clone, Copy)]
pub struct PPUState {
	pub cycle: u64,
	pub maxed: bool,
	pub paused: bool,
	pub window_line: u8,
}

pub trait PPU {
	fn get_mode(&self) -> PPUMode;
	fn get_ly(&self) -> u8;
	fn set_ly(&mut self, value: u8);
	fn set_mode(&mut self, mode: PPUMode);
	fn step_ppu(&mut self, lcd: &mut dyn LCDDisplay);
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
			self.io_register_state[INTERRUPT_REQUEST] |= INT_LCD_STAT;
		}
	}

	fn set_mode(&mut self, _mode: PPUMode) {
		// let state = self.read(IORegistersAddress::STAT as u16);
		// self.write(
		// IORegistersAddress::STAT as u16,
		// (state & 0b11111100) | _mode as u8,
		// );
	}

	fn step_ppu(&mut self, lcd: &mut dyn LCDDisplay) {
		if self.get_ly() < 144 {
			self.render_scanline(lcd, self.get_ly());
		}

		self.set_ly(self.get_ly() + 1);
		self.ppu_state.paused = false;

		if self.get_ly() == 144 {
			self.set_mode(PPUMode::VBlank);
		}

		if self.get_ly() >= 153 {
			if self.ppu_state.maxed {
				self.set_ly(0);
				self.ppu_state.maxed = false;
				self.ppu_state.cycle += 908;
				let interrupt_state = self.read(INTERRUPT_REQUEST);
				self.write(INTERRUPT_REQUEST, interrupt_state | INT_V_BLANK);
				self.set_mode(PPUMode::OamScan)
			} else {
				self.ppu_state.cycle += 4;
				self.ppu_state.maxed = true;
			}
		} else {
			self.ppu_state.cycle += 456;
		}
	}
}
