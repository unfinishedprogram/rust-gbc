use instant::Instant;
use log::debug;

use crate::emulator::{
	flags,
	flags::{get_bit_flag, set_bit_flag, set_bit_flag_to, BitFlag, STATFlag},
	io_registers::IORegistersAdress,
	memory_mapper::MemoryMapper,
};

use super::{lcd::LCDDisplay, renderer::Renderer, EmulatorState};

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
		let num = self.read(IORegistersAdress::STAT as u16) & 0b00000011;
		match num {
			0 => PPUMode::HBlank,
			1 => PPUMode::VBlank,
			2 => PPUMode::OamScan,
			3 => PPUMode::Draw,
			_ => unreachable!(), // Since we only take the last two bits
		}
	}

	fn get_ly(&self) -> u8 {
		self.read(IORegistersAdress::LY as u16)
	}

	fn set_ly(&mut self, value: u8) {
		let lyc_status = self.read(IORegistersAdress::LY as u16) == value;
		self.write(IORegistersAdress::LY as u16, value);
		set_bit_flag_to(self, BitFlag::Stat(STATFlag::LYCeqLY), lyc_status);

		if lyc_status && get_bit_flag(self, BitFlag::Stat(STATFlag::LYCeqLUInterruptEnable)) {
			set_bit_flag(
				self,
				BitFlag::InterruptRequest(flags::InterruptFlag::LcdStat),
			);
		}
	}

	fn set_mode(&mut self, _mode: PPUMode) {}

	fn step_ppu(&mut self, lcd: &mut dyn LCDDisplay) {
		self.set_ly(self.get_ly() + 1);
		self.ppu_state.paused = false;

		if self.get_ly() >= 153 {
			if self.ppu_state.maxed {
				self.set_ly(0);

				// let start = Instant::now();
				self.render(lcd);
				// debug!("Took: {:?}", start.elapsed());

				self.ppu_state.maxed = false;
				self.ppu_state.cycle += 908;
			} else {
				self.ppu_state.cycle += 4;
				self.ppu_state.maxed = true;
			}
		} else {
			self.ppu_state.cycle += 456;
		}
	}
}
