use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TimerState {
	pub timer_clock: u64,
	pub div_clock: u8,
}
