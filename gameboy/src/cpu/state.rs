use serde::{Deserialize, Serialize};

use super::{CPURegisters, flags::Flags};

// TODO: Accurate interrupt handling

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CPUState {
	pub registers: CPURegisters,
	pub halted:bool,
	pub ime: bool,
	pub ie_register: u8,
	pub ie_next: bool,
}

impl Flags for CPUState {
    fn get_flag_byte_mut(&mut self) -> &mut u8 {
		&mut self.registers.bytes[5]
    }

    fn get_flag_byte(&self) -> &u8 {
		&self.registers.bytes[5]
    }
}
