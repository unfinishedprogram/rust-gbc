use sm83::{registers::CPURegister8, values::ValueRefU8, Instruction};
use test_generator::test_resources;

use super::util::rom_loader::init_emulator_with_rom_dmg;
use crate::test::util::rom_loader::init_emulator_with_rom_cgb;

#[test_resources("../test_data/mooneye-test-suite/**/*.gb")]
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

	match state.cpu_state.registers.bytes {
		[_, _, 3, 5, 8, 13, 21, 34] => {} // Success code
		[_, _, 66, 66, 66, 66, 66, 66] => panic!("Test failed with code"), // Failure code
		_ => panic!(
			"Test Failed, no code: {:?}",
			state.cpu_state.registers.bytes
		), // Run untill success or failure
	}
}
