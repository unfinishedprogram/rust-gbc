use sm83::{registers::CPURegister8, values::ValueRefU8, Instruction};
use test_generator::test_resources;

use super::util::{rom_loader::init_emulator_with_rom_cgb, success_code::get_fib_test_result};

#[test_resources("../test_data/age-test-roms/**/*.gb")]
fn age_test(src: &str) {
	let mut state = init_emulator_with_rom_cgb(src);

	for _ in 0..1_048_576 * 10 {
		if let Some(Instruction::LD_8(
			ValueRefU8::Reg(CPURegister8::B),
			ValueRefU8::Reg(CPURegister8::B),
		)) = state.step()
		{
			break;
		}
	}

	get_fib_test_result(&state).unwrap();
}
