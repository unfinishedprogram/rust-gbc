mod cpu;
mod memory_mapper;
mod state;

use self::cpu::MockCpu;
use std::fs::{self, read_dir, DirEntry};

use crate::test::opcode_tests::state::{OpcodeTest, TestState};
use crate::SM83;

#[test]
pub fn run_opcode_tests() {
	let folder = read_dir("./src/test/opcode_tests/v1").unwrap();
	let files = folder.flatten().collect::<Vec<DirEntry>>();

	for file in files {
		let val = fs::read_to_string(file.path()).unwrap();
		let tests: Vec<OpcodeTest> = serde_json::from_str(&val).unwrap();
		for test in tests {
			let mut cpu: MockCpu = test.initial_state.clone().into();
			cpu.step_cpu();
			let end_state: TestState = cpu.into();
			if end_state != test.final_state {
				println!("FAILED:{:}", test.name);
			}
		}
	}
}
