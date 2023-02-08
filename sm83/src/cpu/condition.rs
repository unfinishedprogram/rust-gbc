use std::fmt::{Debug, Formatter, Result};

#[derive(Clone, Copy)]
pub enum Condition {
	NZ,
	Z,
	NC,
	C,
	Always,
}

pub use Condition::*;

impl Debug for Condition {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			NZ => write!(f, "nz"),
			Z => write!(f, "z"),
			NC => write!(f, "nc"),
			C => write!(f, "c"),
			Always => write!(f, ""),
		}
	}
}
