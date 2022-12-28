use crate::screenshot_tests;
use crate::test::utils::screenshot_test::run_screenshot_test;

screenshot_tests! {
	cpu_instrs:("blargg/cpu_instrs", 55),
	dmg_sound:("blargg/dmg_sound", 36),
	halt_bug:("blargg/halt_bug", 2),
	instr_timing:("blargg/instr_timing", 1),
	interrupt_time:("blargg/interrupt_time", 2),
	mem_timing:("blargg/mem_timing", 3),
	oam_bug:("blargg/oam_bug", 4),
	dmg_acid2:("dmg-acid2", 21),
}
