use serde::{Deserialize, Serialize};

pub enum TickResult {
	None,
	LengthCtrl,
	VolumeEnv,
	LengthCtrlAndSweep,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct FrameSequencer {
	pub value: u8,
}

impl FrameSequencer {
	fn value_to_tick_result(value: u8) -> TickResult {
		use TickResult as R;
		match value & 7 {
			0 | 4 => R::LengthCtrl,
			2 | 6 => R::LengthCtrlAndSweep,
			7 => R::VolumeEnv,
			1 | 3 | 5 => R::None,
			0x8..=u8::MAX => unreachable!(),
		}
	}

	pub fn next_result(&self) -> TickResult {
		Self::value_to_tick_result(self.value.wrapping_add(1))
	}

	pub fn current_result(&self) -> TickResult {
		Self::value_to_tick_result(self.value)
	}

	pub fn tick(&mut self) -> TickResult {
		self.value = self.value.wrapping_add(1);
		self.current_result()
	}
}
