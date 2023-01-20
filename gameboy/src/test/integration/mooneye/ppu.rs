use crate::mooneye_tests;
use crate::test::util::mooneye_test::run_mooneye_test;

mooneye_tests! {
	intr_2_0_timing:"mooneye/acceptance/ppu/intr_2_0_timing",
	intr_2_mode0_timing:"mooneye/acceptance/ppu/intr_2_mode0_timing",
	intr_2_mode0_timing_sprites:"mooneye/acceptance/ppu/intr_2_mode0_timing_sprites",
	intr_2_mode3_timing:"mooneye/acceptance/ppu/intr_2_mode3_timing",
	intr_2_oam_ok_timing:"mooneye/acceptance/ppu/intr_2_oam_ok_timing",
	lcdon_timing_gs:"mooneye/acceptance/ppu/lcdon_timing-GS",
	lcdon_write_timing_gs:"mooneye/acceptance/ppu/lcdon_write_timing-GS",
	stat_irq_blocking:"mooneye/acceptance/ppu/stat_irq_blocking",
	stat_lyc_onoff:"mooneye/acceptance/ppu/stat_lyc_onoff",
	vblank_stat_intr_gs:"mooneye/acceptance/ppu/vblank_stat_intr-GS",
}
