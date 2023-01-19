use crate::mooneye_tests;
use crate::test::util::mooneye_test::run_mooneye_test;

mooneye_tests! {
	// MBC1
	mbc1_bits_bank1: "mooneye/mbc1/bits_bank1",
	mbc1_bits_bank2: "mooneye/mbc1/bits_bank2",
	mbc1_bits_mode: "mooneye/mbc1/bits_mode",
	mbc1_bits_ramg: "mooneye/mbc1/bits_ramg",
	// mbc1_multicart_rom_8_mb: "mooneye/mbc1/multicart_rom_8Mb",
	mbc1_ram_256kb: "mooneye/mbc1/ram_256kb",
	mbc1_ram_64kb: "mooneye/mbc1/ram_64kb",
	mbc1_rom_16_mb: "mooneye/mbc1/rom_16Mb",
	mbc1_rom_1_mb: "mooneye/mbc1/rom_1Mb",
	mbc1_rom_2_mb: "mooneye/mbc1/rom_2Mb",
	mbc1_rom_4_mb: "mooneye/mbc1/rom_4Mb",
	mbc1_rom_512kb: "mooneye/mbc1/rom_512kb",
	mbc1_rom_8_mb: "mooneye/mbc1/rom_8Mb",

	// MBC2
	mbc2_bits_ramg: "mooneye/mbc2/bits_ramg",
	mbc2_bits_romb: "mooneye/mbc2/bits_romb",
	mbc2_bits_unused: "mooneye/mbc2/bits_unused",
	mbc2_ram: "mooneye/mbc2/ram",
	mbc2_rom_1_mb: "mooneye/mbc2/rom_1Mb",
	mbc2_rom_2_mb: "mooneye/mbc2/rom_2Mb",
	mbc2_rom_512kb: "mooneye/mbc2/rom_512kb",


	// MBC5
	rom_16_mb: "mooneye/mbc5/rom_16Mb",
	rom_1_mb: "mooneye/mbc5/rom_1Mb",
	rom_2_mb: "mooneye/mbc5/rom_2Mb",
	rom_32_mb: "mooneye/mbc5/rom_32Mb",
	rom_4_mb: "mooneye/mbc5/rom_4Mb",
	rom_512_kb: "mooneye/mbc5/rom_512kb",
	rom_64_mb: "mooneye/mbc5/rom_64Mb",
	rom_8_mb: "mooneye/mbc5/rom_8Mb",
}
