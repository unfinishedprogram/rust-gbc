use super::CPURegisters;

#[derive(Clone, Default)]
pub struct CPUState {
	pub registers: CPURegisters,
	pub interrupt_enable: bool,
	pub t_states: u64,
	pub ie_next: bool,
	pub ie_next_next: bool,
}
