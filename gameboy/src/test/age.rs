use sm83::{registers::CPURegister8, values::ValueRefU8, Instruction};
use test_generator::test_resources;

use super::util::{rom_loader::init_emulator_with_rom_dmg, success_code::test_fib_success_code};

#[test_resources("../test_data/age-test-roms/**/*.gb")]
fn age_test(src: &str) {
	let mut state = init_emulator_with_rom_dmg(src);

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
