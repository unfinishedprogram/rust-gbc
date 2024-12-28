use super::util::rom_loader::{init_emulator_with_rom_cgb, init_emulator_with_rom_dmg};
use crate::{test::util::screenshot_test::compare_lcd, Gameboy, Mode};
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

	panic!(
		"Images do not match at frame: {} as {:?}",
		state.ppu.frame,
		if matches!(state.mode, Mode::DMG) {
			"DMG"
		} else {
			"CGB"
		}
	);
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

fn run_test_dmg(name: &str, postfix: &str) {
	let (rom_path, img_path) = load_test_data(name, postfix);
	let gb = init_emulator_with_rom_dmg(&rom_path);
	execute_blargg_test(
		&format!("{}-DMG", name),
		gb,
		image::open(img_path).unwrap().into_rgba8(),
	)
}

fn run_test_cgb(name: &str, postfix: &str) {
	let (rom_path, img_path) = load_test_data(name, postfix);
	let gb = init_emulator_with_rom_cgb(&rom_path);
	execute_blargg_test(
		&format!("{}-CGB", name),
		gb,
		image::open(img_path).unwrap().into_rgba8(),
	)
}

#[test]
fn dmg_sound() {
	run_test_cgb("dmg_sound", "-dmg");
}

#[test]
fn cgb_sound() {
	run_test_cgb("cgb_sound", "-cgb");
}

#[test]
fn dmg_cpu_instrs() {
	run_test_dmg("cpu_instrs", "-dmg-cgb");
}

#[test]
fn cgb_cpu_instrs() {
	run_test_cgb("cpu_instrs", "-dmg-cgb");
}

#[test]
fn dmg_instr_timing() {
	run_test_dmg("instr_timing", "-dmg-cgb");
}

#[test]
fn cgb_instr_timing() {
	run_test_cgb("instr_timing", "-dmg-cgb");
}

#[test]
fn dmg_interrupt_time() {
	run_test_dmg("instr_timing", "-dmg");
}

#[test]
fn cgb_interrupt_time() {
	run_test_cgb("instr_timing", "-cgb");
}

#[test]
fn dmg_mem_timing() {
	run_test_dmg("mem_timing", "-dmg-cgb");
}

#[test]
fn cgb_mem_timing() {
	run_test_cgb("mem_timing", "-dmg-cgb");
}

#[test]
fn dmg_oam_bug() {
	run_test_dmg("oam_bug", "-dmg");
}

#[test]
fn cgb_oam_bug() {
	run_test_cgb("oam_bug", "-cgb");
}
