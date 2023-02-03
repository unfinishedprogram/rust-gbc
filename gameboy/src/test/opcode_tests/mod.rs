mod cpu;
mod memory_mapper;
mod state;

use core::panic;
use std::{
	cmp::Ordering,
	fs::{self, read_dir, DirEntry},
	io::Error,
};

use crate::{
	cpu::CPU,
	test::opcode_tests::state::{OpcodeTest, TestState},
};

use self::cpu::MockCpu;

#[test]
pub fn run_opcode_tests() {
	let folder = read_dir("./src/test/opcode_tests/v1").unwrap();
	let mut files = folder.flatten().collect::<Vec<DirEntry>>();

	let mut individual_ran: u32 = 0;
	let mut individual_passed: u32 = 0;
	let mut opcodes_ran: u32 = 0;

	for file in files {
		let val = fs::read_to_string(file.path()).unwrap();
		let tests: Vec<OpcodeTest> = serde_json::from_str(&val).unwrap();
		for test in tests {
			individual_ran += 1;

			let mut cpu: MockCpu = test.initial_state.clone().into();
			cpu.step_cpu();
			let end_state: TestState = cpu.into();
			if end_state != test.final_state {
				println!("Start: \n{:?}", test.initial_state);
				println!("FAILED:{:}", test.name);
				panic!("\n{end_state:?} NOT EQUAL \n{:?}", test.final_state);
			} else {
				individual_passed += 1;
			}
		}
		opcodes_ran += 1;
		// println!("{opcodes_ran}/500");
	}
	// panic!(
	// 	"Ran:{individual_ran:}, Passed:{individual_passed:}, {}%",
	// 	(individual_passed as f32) / (individual_ran as f32)
	// )
}
