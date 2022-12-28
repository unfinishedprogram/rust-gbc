use serde::Serialize;

#[derive(Default, Clone, Serialize)]
pub struct TimerState {
	pub timer_clock: u64,
	pub div_clock: u64,
}
