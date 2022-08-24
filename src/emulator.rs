pub struct EmulatorState {
	pub memory: [u8; 0xFFFF],
}
#[repr(C, packed)]
pub struct Memory {
}

impl EmulatorState {
	pub fn new() -> Self {
		Self {
			memory:[0; 0xFFFF],
		}
	}
}

