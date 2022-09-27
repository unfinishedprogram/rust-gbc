#[derive(Copy, Clone, Debug)]
pub enum Condition {
	NZ,
	Z,
	NC,
	C,
	ALWAYS,
}
