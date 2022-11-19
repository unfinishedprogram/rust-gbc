use super::CPURegisters;

#[derive(Clone, Copy)]
pub struct CPUState {
	pub registers: CPURegisters,
	pub interrupt_enable: bool,
	pub t_states: u64,
	pub ie_next: bool,
	pub ie_next_next: bool,
}

impl Default for CPUState {
	fn default() -> CPUState {
		CPUState {
			registers: CPURegisters::default(),
			interrupt_enable: false,
			t_states: 0,
			ie_next: false,
			ie_next_next: false,
		}
	}
}
