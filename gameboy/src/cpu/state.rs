use serde::{Deserialize, Serialize};

use super::CPURegisters;

// TODO: Accurate interrupt handling

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CPUState {
	pub registers: CPURegisters,
	pub ime: bool,
	pub ie_next: bool,
}
