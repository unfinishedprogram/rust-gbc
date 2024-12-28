use sm83::registers::{Addressable, CPURegister8};

use crate::Gameboy;

#[derive(Debug)]
pub enum FailureCode {
	Failure,
	Unknown([u8; 6]),
}

// Tests for the fibonacci sequence in registers
// indicating a passing test for mooneye, and same-suite tests
pub fn test_fib_success_code(gb: &Gameboy) -> Result<(), FailureCode> {
	let bytes = [
		gb.cpu_state.read(CPURegister8::B),
		gb.cpu_state.read(CPURegister8::C),
		gb.cpu_state.read(CPURegister8::D),
		gb.cpu_state.read(CPURegister8::E),
		gb.cpu_state.read(CPURegister8::H),
		gb.cpu_state.read(CPURegister8::L),
	];

	match bytes {
		[3, 5, 8, 13, 21, 34] => Ok(()),
		[66, 66, 66, 66, 66, 66] => Err(FailureCode::Failure),
		_ => Err(FailureCode::Unknown(bytes)),
	}
}
