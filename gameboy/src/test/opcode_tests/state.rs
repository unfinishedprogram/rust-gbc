use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TestState {
	pub pc: u16,
	pub sp: u16,
	pub a: u8,
	pub b: u8,
	pub c: u8,
	pub d: u8,
	pub e: u8,
	pub f: u8,
	pub h: u8,
	pub l: u8,
	// pub ime: u8,
	pub ram: Vec<(u16, u8)>,
	// pub ei: u8,
}

#[derive(Deserialize)]
pub struct OpcodeTest {
	pub name: String,
	#[serde(alias = "initial")]
	pub initial_state: TestState,
	#[serde(alias = "final")]
	pub final_state: TestState,
}
