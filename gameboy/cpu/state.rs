use serde::{Deserialize, Serialize};

use super::CPURegisters;

// TODO: Accurate interrupt handling

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CPUState {
	pub registers: CPURegisters,
	pub interrupt_enable: bool,
	ie_next: bool,
	ie_next_next: bool,
}
