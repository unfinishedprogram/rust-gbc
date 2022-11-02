use std::fmt::Debug;

#[derive(Copy, Clone)]
pub enum Condition {
	NZ,
	Z,
	NC,
	C,
	ALWAYS,
}

impl Debug for Condition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NZ => write!(f, "NZ"),
			Self::Z => write!(f, "Z"),
			Self::NC => write!(f, "NC"),
			Self::C => write!(f, "C"),
			Self::ALWAYS => write!(f, ""),
		}
	}
}
