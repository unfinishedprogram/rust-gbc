use std::default;

use super::CPURegisters;

pub struct CPUState {
	pub registers: CPURegisters,
	pub interrupt_enable: bool,
}

impl Default for CPUState {
	fn default() -> Self {
		Self {
			registers: CPURegisters::default(),
			interrupt_enable: true,
		}
	}
}
