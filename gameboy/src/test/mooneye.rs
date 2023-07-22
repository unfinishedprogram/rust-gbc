use test_generator::test_resources;

use crate::test::util::rom_loader::init_emulator_with_rom_cgb;

#[test_resources("../test_data/mooneye-test-suite/*/*.gb")]
fn mooneye_test(rom: &str) {
	let mut state = init_emulator_with_rom_cgb(rom);
	for _ in 0..40 {
		for _ in 0..1_048_576 {
			state.step();
		}
		match state.cpu_state.registers.bytes {
			[_, 3, 5, 8, 13, _, 21, 34] => return,   // Success code
			[_, 66, 66, 66, 66, _, 66, 66] => break, // Failure code
			_ => {}                                  // Run untill success or failure
		}
	}
	panic!("Test Failed")
}
