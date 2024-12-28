use sm83::{registers::CPURegister8, values::ValueRefU8, Instruction};
use test_generator::test_resources;

use super::util::success_code::get_fib_test_result;
use crate::test::util::rom_loader::init_emulator_with_rom_cgb;

fn is_gbc_compatible_test(path: &str) -> bool {
	let name = path.split('/').last().unwrap().split(".gb").next().unwrap();

	// No hardware specified
	if !name.contains('-') {
		return true;
	}

	let flags_str = name.split('-').last().unwrap();

	flags_str.contains("C") && flags_str.chars().all(char::is_uppercase)
		|| flags_str.contains("cgb")
}

fn mooneye_test(rom: &str) {
	if !is_gbc_compatible_test(rom) {
		return;
	}

	let mut state = init_emulator_with_rom_cgb(rom);

	for _ in 0..1_048_576 * 100 {
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

#[test_resources("../test_data/mooneye-test-suite/emulator-only/**/*.gb")]
fn emulator_only(rom: &str) {
	mooneye_test(rom)
}

#[test_resources("../test_data/mooneye-test-suite/acceptance/**/*.gb")]
fn acceptance(rom: &str) {
	mooneye_test(rom)
}
