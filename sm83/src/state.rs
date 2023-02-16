use serde::{Deserialize, Serialize};

use crate::cpu::flags::Flags;

use super::registers::{CPURegister8, CPURegisters};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CPUState {
	pub registers: CPURegisters,
	pub halted: bool,
	pub ime: bool,
	pub ie_register: u8,
	pub ie_next: bool,
}

impl Flags for CPUState {
	fn get_flag_byte_mut(&mut self) -> &mut u8 {
		&mut self.registers[CPURegister8::F]
	}

	fn get_flag_byte(&self) -> &u8 {
		&self.registers[CPURegister8::F]
	}
}
