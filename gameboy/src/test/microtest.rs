use sm83::memory_mapper::MemoryMapper;
use test_generator::test_resources;

use super::util::rom_loader::init_emulator_with_rom_dmg;

#[test_resources("../test_data/gbmicrotest/*.gb")]
fn microtest(src: &str) {
	let mut state = init_emulator_with_rom_dmg(src);

	for _ in 0..1000 * 100 {
		state.step();
	}

	let result = state.read(0xFF80);
	let expected = state.read(0xFF81);
	let passed = state.read(0xFF82);

	assert_eq!(result, expected);
	assert_eq!(passed, 0x01)
}
