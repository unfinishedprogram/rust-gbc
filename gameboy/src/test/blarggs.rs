use super::util::rom_loader::init_emulator_with_rom;
use crate::{test::util::screenshot_test::compare_lcd, Gameboy};
use image::{EncodableLayout, RgbaImage};
use std::fs::create_dir_all;

pub fn execute_blargg_test(name: &str, mut state: Gameboy, img: RgbaImage) {
	let expected = img.as_bytes();

	let mut actual: Vec<u8> = vec![];

	for _ in 0..32 {
		for _ in 0..1_048_576 {
			state.step();
		}

		actual = state.ppu.lcd.front_buffer().to_vec();

		if compare_lcd(&actual, expected) {
			return;
		}
	}

	let actual = RgbaImage::from_raw(160, 144, actual).unwrap();

	save_failed_image_result(name, &actual, &img);

	panic!("Images do not match at frame: {}", state.ppu.frame);
}

fn save_failed_image_result(name: &str, actual: &RgbaImage, expected: &RgbaImage) {
	create_dir_all(format!("failed_results/{name}")).unwrap();

	actual
		.save(format!("failed_results/{name}/actual.png"))
		.unwrap();

	expected
		.save(format!("failed_results/{name}/expected.png"))
		.unwrap();
}

fn load_test_data(name: &str, img_postfix: &str) -> (String, String) {
	let rom_path = format!("../test_data/blargg/{name}/{name}.gb");
	let img_path = format!("../test_data/blargg/{name}/{name}{img_postfix}.png");

	(rom_path, img_path)
}

fn run_blarggs(name: &str, postfix: &str) {
	let (rom_path, img_path) = load_test_data(name, postfix);
	let gb = init_emulator_with_rom(&rom_path);
	execute_blargg_test(
		&format!("{name}-CGB"),
		gb,
		image::open(img_path).unwrap().into_rgba8(),
	)
}

#[test]
fn cgb_sound() {
	run_blarggs("cgb_sound", "-dmg-cgb");
}

#[test]
fn cpu_instrs() {
	run_blarggs("cpu_instrs", "-dmg-cgb");
}

#[test]
fn instr_timing() {
	run_blarggs("instr_timing", "-dmg-cgb");
}

#[test]
fn interrupt_time() {
	run_blarggs("interrupt_time", "-cgb");
}

#[test]
fn mem_timing() {
	run_blarggs("mem_timing", "-dmg-cgb");
}

#[test]
fn oam_bug() {
	run_blarggs("oam_bug", "-cgb");
}
