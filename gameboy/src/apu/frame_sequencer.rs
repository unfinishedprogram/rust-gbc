use serde::{Deserialize, Serialize};

pub enum TickResult {
	LengthCtrl,
	VolumeEnv,
	LengthCtrlAndSweep,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct FrameSequencer {
	pub value: u8,
}

impl FrameSequencer {
	pub fn tick(&mut self) -> Option<TickResult> {
		use TickResult as R;

		self.value = self.value.wrapping_add(1);

		match self.value & 7 {
			0 | 4 => Some(R::LengthCtrl),
			2 | 6 => Some(R::LengthCtrlAndSweep),
			7 => Some(R::VolumeEnv),
			1 | 3 | 5 => None,
			0x8..=u8::MAX => unreachable!(),
		}
	}
}
