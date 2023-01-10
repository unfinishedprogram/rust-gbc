use serde::{Deserialize, Serialize};

use super::CPURegisters;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CPUState {
	pub registers: CPURegisters,
	pub interrupt_enable: bool,
	pub ie_next: bool,
	pub ie_next_next: bool,
}
