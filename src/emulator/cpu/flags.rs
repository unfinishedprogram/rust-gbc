use crate::emulator::state::EmulatorState;

use super::{registers::CPURegister8, CPU};

pub enum Flag {
	Z = 0b10000000,
	N = 0b01000000,
	H = 0b00100000,
	C = 0b00010000,
}

pub trait Flags {
	fn get_flag_byte(&self) -> u8;
	fn set_flag_byte(&mut self, byte: u8);

	fn set_flag_to(&mut self, flag: Flag, value: bool) {
		if value {
			self.set_flag(flag)
		} else {
			self.clear_flag(flag)
		}
	}

	fn clear_flag(&mut self, flag: Flag) {
		self.set_flag_byte(self.get_flag_byte() & !(flag as u8));
	}
	fn set_flag(&mut self, flag: Flag) {
		self.set_flag_byte(self.get_flag_byte() | flag as u8);
	}
	fn get_flag(&self, flag: Flag) -> bool {
		self.get_flag_byte() & flag as u8 != 0
	}
}

impl Flags for EmulatorState {
	fn get_flag_byte(&self) -> u8 {
		return self.cpu_state.registers[CPURegister8::F];
	}

	fn set_flag_byte(&mut self, byte: u8) {
		self.write_8(&CPURegister8::F.into(), byte);
	}
}
