use crate::mooneye_tests;
use crate::test::util::mooneye_test::run_mooneye_test;

mooneye_tests! {
	basic: "mooneye/acceptance/oam_dma/basic",
	reg_read: "mooneye/acceptance/oam_dma/reg_read",
	sources: "mooneye/acceptance/oam_dma/sources",
}
