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
			Self::NZ => write!(f, "nz"),
			Self::Z => write!(f, "z"),
			Self::NC => write!(f, "nc"),
			Self::C => write!(f, "c"),
			Self::ALWAYS => write!(f, ""),
		}
	}
}
