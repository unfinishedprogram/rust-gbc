use serde::{Deserialize, Serialize};

use crate::{
	cpu::{flags::Flags, interrupt::Interrupt},
	registers::{Addressable, CPURegister16},
};

use super::registers::{CPURegister8, CPURegisters};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CPUState {
	registers: CPURegisters,
	pub halted: bool,
	interrupt_master_enable: bool,
	pub interrupt_enable: u8,  // IE
	pub interrupt_request: u8, // IF
	ie_next: bool,
}

impl Flags for CPUState {
	fn read_flag_byte(&self) -> u8 {
		self.read(CPURegister8::F)
	}

	fn write_flag_byte(&mut self, value: u8) {
		self.write(CPURegister8::F, value)
	}
}

impl CPUState {
	fn clear_interrupt_request(&mut self, interrupt: u8) {
		self.interrupt_request &= !interrupt;
	}

	pub fn interrupt_pending(&self) -> bool {
		self.interrupt_request & self.interrupt_enable != 0
	}

	pub fn disable_interrupts(&mut self) {
		self.interrupt_master_enable = false;
		self.ie_next = false;
	}

	pub fn enable_interrupts(&mut self) {
		self.ie_next = true;
	}

	pub fn consume_next_interrupt(&mut self) -> Option<Interrupt> {
		if !self.interrupt_master_enable {
			return None;
		}

		let requests = self.interrupt_enable & self.interrupt_request;
		if requests != 0 {
			// Gets the rightmost set bit
			let index = requests & (!requests + 1);
			self.clear_interrupt_request(index);
			index.try_into().ok()
		} else {
			None
		}
	}

	pub fn ime(&self) -> bool {
		self.interrupt_master_enable
	}

	pub fn tick_ie_delay(&mut self) {
		self.interrupt_master_enable = self.ie_next;
	}
}

impl Addressable<CPURegister8, u8> for CPUState {
	fn read(&self, index: CPURegister8) -> u8 {
		self.registers.read(index)
	}

	fn write(&mut self, index: CPURegister8, value: u8) {
		self.registers.write(index, value)
	}
}

impl Addressable<CPURegister16, u16> for CPUState {
	fn read(&self, index: CPURegister16) -> u16 {
		self.registers.read(index)
	}

	fn write(&mut self, index: CPURegister16, value: u16) {
		self.registers.write(index, value)
	}
}
