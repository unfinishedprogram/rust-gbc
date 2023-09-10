use sm83::{registers::CPURegister8, values::ValueRefU8, Instruction};
use test_generator::test_resources;

use crate::test::util::rom_loader::init_emulator_with_rom_cgb;

#[test_resources("../test_data/mooneye-test-suite/*/*.gb")]
fn mooneye_test(rom: &str) {
	let mut state = init_emulator_with_rom_cgb(rom);
	for _ in 0..1_048_576 * 40 {
		if let Some(Instruction::LD_8(
			ValueRefU8::Reg(CPURegister8::B),
			ValueRefU8::Reg(CPURegister8::B),
		)) = state.step()
		{
			break;
		}
	}

	match state.cpu_state.registers.bytes {
		[_, 3, 5, 8, 13, _, 21, 34] => {} // Success code
		[_, 66, 66, 66, 66, _, 66, 66] => panic!("Test failed with code"), // Failure code
		_ => panic!("Test Failed, no code"), // Run untill success or failure
	}
}
