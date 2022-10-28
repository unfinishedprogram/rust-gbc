use super::CPURegisters;

pub struct CPUState {
	pub registers: CPURegisters,
	pub interrupt_enable: bool,
	pub t_states: u64,
}

impl Default for CPUState {
	fn default() -> CPUState {
		CPUState {
			registers: CPURegisters::default(),
			interrupt_enable: false,
			t_states: 0,
		}
	}
}
