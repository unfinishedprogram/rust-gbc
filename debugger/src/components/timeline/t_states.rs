#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TStates(u64);

impl TStates {
	#[inline(always)]
	pub fn from_t_states(t_states: u64) -> TStates {
		TStates(t_states)
	}

	#[inline(always)]
	pub fn from_seconds(seconds: f64) -> TStates {
		TStates((seconds * 4.0 * 1_048_576.0) as u64)
	}

	#[inline(always)]
	pub fn from_m_states(m_states: u64) -> TStates {
		TStates(m_states * 4)
	}

	#[inline(always)]
	pub fn from_frames(frames: f64) -> TStates {
		TStates((frames * 70224.0) as u64)
	}

	#[inline(always)]
	pub fn t_states(self) -> u64 {
		self.0
	}

	#[inline(always)]
	pub fn seconds(self) -> f64 {
		self.0 as f64 / 4.0 / 1_048_576.0
	}

	#[inline(always)]
	pub fn m_states(self) -> u64 {
		self.0 / 4
	}

	#[inline(always)]
	pub fn frames(self) -> f64 {
		self.0 as f64 / 70224.0
	}
}

impl From<TStates> for u64 {
	fn from(val: TStates) -> Self {
		val.0
	}
}

impl From<u64> for TStates {
	fn from(val: u64) -> Self {
		Self(val)
	}
}
