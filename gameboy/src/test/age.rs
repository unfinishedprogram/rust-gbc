use std::fs::read;

use sm83::{registers::CPURegister8, values::ValueRefU8, Instruction};
use test_generator::test_resources;

use super::util::success_code::test_fib_success_code;
use crate::Gameboy;

#[test_resources("../test_data/age-test-roms/**/*.gb")]
fn age_test(src: &str) {
	let mut state = Gameboy::cgb();
	let rom = read(src).unwrap();
	state.load_rom(&rom, None);
	state.run_until_boot();

	for _ in 0..1_048_576 * 100 {
		if let Some(Instruction::LD_8(
			ValueRefU8::Reg(CPURegister8::B),
			ValueRefU8::Reg(CPURegister8::B),
		)) = state.step()
		{
			break;
		}
	}

	test_fib_success_code(&state).unwrap();
}
