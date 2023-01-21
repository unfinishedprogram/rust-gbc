// https://github.com/c-sp/gameboy-test-roms/blob/master/src/howto/mooneye-test-suite.md

use super::rom_loader::init_emulator_with_rom;

pub fn run_mooneye_test(rom: &str) {
	let mut state = init_emulator_with_rom(rom);

	for _ in 0..20 {
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

#[macro_export]
macro_rules! mooneye_tests {
    ($($name:ident: $value:expr,)*) => {
		$(
			#[test]
			fn $name() {
				let rom = format!("../roms/test/{}.gb", $value);
				run_mooneye_test(&rom);
			}
		)*
    }
}
