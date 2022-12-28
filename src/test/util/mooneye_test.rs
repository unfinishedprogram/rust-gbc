// https://github.com/c-sp/gameboy-test-roms/blob/master/src/howto/mooneye-test-suite.md

use super::rom_loader::init_emulator_with_rom;

pub fn run_mooneye_test(rom: &str) {
	let mut state = init_emulator_with_rom(rom);

	for _ in 0..120 {
		for _ in 0..1_048_576 {
			state.step();
		}
		if let [_, 3, 5, 8, 13, _, 21, 34] = state.cpu_state.registers.bytes {
			return;
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
				let rom = format!("roms/test/{}.gb", $value);
				run_mooneye_test(&rom);
			}
		)*
    }
}
