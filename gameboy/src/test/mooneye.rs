use sm83::{registers::CPURegister8, values::ValueRefU8, Instruction};
use test_generator::test_resources;

use super::util::{rom_loader::init_emulator_with_rom_dmg, success_code::test_fib_success_code};
use crate::test::util::rom_loader::init_emulator_with_rom_cgb;

#[test_resources("../test_data/mooneye-test-suite/emulator-only/**/*.gb")]
fn emulator_only(rom: &str) {
	mooneye_test(rom)
}

#[test_resources("../test_data/mooneye-test-suite/acceptance/**/*.gb")]
fn acceptance(rom: &str) {
	mooneye_test(rom)
}

fn mooneye_test(rom: &str) {
	if rom.contains("mgb")
		|| rom.contains("sgb")
		|| rom.contains("sgb2")
		|| rom.contains("agb")
		|| rom.contains("ags")
	{
		return;
	}

	let mut state = if rom.contains("dmg") {
		init_emulator_with_rom_dmg(rom)
	} else {
		init_emulator_with_rom_cgb(rom)
	};

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
