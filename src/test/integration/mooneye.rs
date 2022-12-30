use crate::mooneye_tests;
use crate::test::util::mooneye_test::run_mooneye_test;

pub mod boot;
pub mod instruction_timing;
pub mod memory_bank_controllers;
pub mod oam_dma;
pub mod timer;

mooneye_tests! {
	dda:"mooneye/acceptance/daa",
	ie_push:"mooneye/acceptance/ie_push",
}
