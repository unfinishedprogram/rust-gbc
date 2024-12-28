use sm83::{registers::CPURegister8, values::ValueRefU8, Instruction};
use test_generator::test_resources;

use super::util::{rom_loader::init_emulator_with_rom_dmg, success_code::get_fib_test_result};
use crate::test::util::rom_loader::init_emulator_with_rom_cgb;

#[test_resources("../test_data/same-suite/**/*.gb")]
fn same_suite_test(rom: &str) {
	let mut state = if rom.contains("dmg") {
		init_emulator_with_rom_dmg(rom)
	} else {
		init_emulator_with_rom_cgb(rom)
	};

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
