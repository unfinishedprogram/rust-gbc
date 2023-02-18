use serde::{Deserialize, Serialize};

use crate::cpu::flags::Flags;

use super::registers::{CPURegister8, CPURegisters};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CPUState {
	pub registers: CPURegisters,
	pub halted: bool,
	pub interrupt_master_enable: bool,
	pub interrupt_enable: u8,
	pub interrupt_request: u8,
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

impl CPUState {
	fn clear_interrupt_request(&mut self, interrupt: u8) {
		self.interrupt_request &= !interrupt;
	}

	pub fn interrupt_pending(&self) -> bool {
		self.interrupt_request & self.interrupt_enable != 0
	}

	pub fn fetch_next_interrupt(&mut self) -> Option<u8> {
		if !self.interrupt_master_enable {
			return None;
		}

		let requests = self.interrupt_enable & self.interrupt_request;
		if requests != 0 {
			// Gets the rightmost set bit
			let index = requests & (!requests + 1);
			self.clear_interrupt_request(index);
			Some(index)
		} else {
			None
		}
	}

	pub fn tick_ie_delay(&mut self) {
		self.interrupt_master_enable = self.ie_next;
	}
}
