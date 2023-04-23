use image::EncodableLayout;

use super::rom_loader::init_emulator_with_rom_cgb;

pub fn compare_lcd(a: &[u8], b: &[u8]) -> bool {
	assert!(
		a.len() == b.len(),
		"Images are not of the same size, a:{}, b:{}",
		a.len(),
		b.len()
	);

	for (a, b) in a.iter().zip(b.iter()) {
		if a != b {
			return false;
		}
	}
	true
}
