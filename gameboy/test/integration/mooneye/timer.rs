use crate::mooneye_tests;
use crate::test::util::mooneye_test::run_mooneye_test;

mooneye_tests! {
	div_write: "mooneye/acceptance/timer/div_write",
	rapid_toggle: "mooneye/acceptance/timer/rapid_toggle",
	tim00: "mooneye/acceptance/timer/tim00",
	tim00_div_trigger: "mooneye/acceptance/timer/tim00_div_trigger",
	tim01: "mooneye/acceptance/timer/tim01",
	tim01_div_trigger: "mooneye/acceptance/timer/tim01_div_trigger",
	tim10: "mooneye/acceptance/timer/tim10",
	tim10_div_trigger: "mooneye/acceptance/timer/tim10_div_trigger",
	tim11: "mooneye/acceptance/timer/tim11",
	tim11_div_trigger: "mooneye/acceptance/timer/tim11_div_trigger",
	tima_reload: "mooneye/acceptance/timer/tima_reload",
	tima_write_reloading: "mooneye/acceptance/timer/tima_write_reloading",
	tma_write_reloading: "mooneye/acceptance/timer/tma_write_reloading",
}
