use crate::app::components::logger;

use crate::emulator::{
	flags,
	flags::{get_bit_flag, set_bit_flag, set_bit_flag_to, BitFlag, STATFlag},
	memory_mapper::MemoryMapper,
	memory_registers::MemoryRegister::*,
};

use super::EmulatorState;

#[derive(Debug)]
pub enum PPUMode {
	HBlank = 0,
	VBlank = 1,
	OamScan = 2,
	Draw = 3,
}

#[derive(Default)]
pub struct PPUState {
	t_states: u64,
}

pub trait PPU {
	fn get_mode(&self) -> PPUMode;
	fn get_ly(&self) -> u8;
	fn set_ly(&mut self, value: u8);
	fn set_mode(&mut self, mode: PPUMode);
	fn step(&mut self);
}

impl PPU for EmulatorState {
	fn get_mode(&self) -> PPUMode {
		let num = self.read(STAT as u16) & 0b00000011;
		return match num {
			0 => PPUMode::HBlank,
			1 => PPUMode::VBlank,
			2 => PPUMode::OamScan,
			3 => PPUMode::Draw,
			_ => PPUMode::HBlank,
		};
	}

	fn get_ly(&self) -> u8 {
		return self.read(LY as u16);
	}

	fn set_ly(&mut self, value: u8) {
		let lyc_status = self.read(LY as u16) == value;
		self.write(LY as u16, value);
		set_bit_flag_to(self, BitFlag::Stat(STATFlag::LYCeqLY), lyc_status);

		if lyc_status && get_bit_flag(self, BitFlag::Stat(STATFlag::LYCeqLUInterruptEnable)) {
			set_bit_flag(
				self,
				BitFlag::InterruptRequest(flags::InterruptFlag::LcdStat),
			);
		}
	}

	fn set_mode(&mut self, mode: PPUMode) {
		use STATFlag::*;
		match mode {
			PPUMode::HBlank => {
				if get_bit_flag(self, BitFlag::Stat(HBlankStatInterruptEnable)) {
					set_bit_flag(
						self,
						BitFlag::InterruptRequest(flags::InterruptFlag::LcdStat),
					);
				}
				self.ppu_state.t_states += 204;
			}
			PPUMode::VBlank => {
				if get_bit_flag(self, BitFlag::Stat(VBlankStatInterruptEnable)) {
					set_bit_flag(
						self,
						BitFlag::InterruptRequest(flags::InterruptFlag::LcdStat),
					);
				}
				self.ppu_state.t_states += 456;
				self.set_ly(self.get_ly() + 1)
			}

			PPUMode::OamScan => {
				if get_bit_flag(self, BitFlag::Stat(OAMStatInterruptEnable)) {
					set_bit_flag(
						self,
						BitFlag::InterruptRequest(flags::InterruptFlag::LcdStat),
					);
				}

				self.ppu_state.t_states += 80;
				if self.get_ly() >= 153 {
					self.set_ly(0);
				} else {
					self.set_ly(self.get_ly() + 1);
				}
			}
			PPUMode::Draw => self.ppu_state.t_states += 172,
		}
		logger::debug(format!("PPU Start: {:?}", mode));

		let current_stat = self.read(STAT as u16);
		self.write(STAT as u16, (current_stat & 0b11111100) | mode as u8);
	}

	fn step(&mut self) {
		if self.ppu_state.t_states == 0 {
			use PPUMode::*;
			match (self.get_mode(), self.get_ly()) {
				(OamScan, _) => self.set_mode(Draw),
				(Draw, _) => self.set_mode(HBlank),
				(HBlank, 0..=143) => self.set_mode(OamScan),
				(HBlank, _) => self.set_mode(VBlank),
				(VBlank, 144..=153) => self.set_mode(VBlank),
				(VBlank, _) => self.set_mode(OamScan),
			}
		}
		self.ppu_state.t_states -= 1;
	}
}
