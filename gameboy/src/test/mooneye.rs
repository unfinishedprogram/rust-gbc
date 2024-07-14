use core::panic;

use bitflags::bitflags;
use sm83::{registers::CPURegister8, values::ValueRefU8, Instruction};
use test_generator::test_resources;

use super::util::{rom_loader::init_emulator_with_rom_dmg, success_code::test_fib_success_code};
use crate::test::util::rom_loader::init_emulator_with_rom_cgb;

bitflags! {
	#[derive(Debug)]
	struct HardwareRevision:u8 {
		const DMG = 1;
		const MGB = 1 << 1;
		const SGB = 1 << 2;
		const SGB2 = 1 << 3;
		const CGB = 1 << 4;
		const AGB = 1 << 5;
		const AGS = 1 << 6;
	}
}

impl TryFrom<&str> for HardwareRevision {
	type Error = ();

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"dmg" => Ok(HardwareRevision::DMG),
			"dmg0" => Ok(HardwareRevision::DMG),
			"mgb" => Ok(HardwareRevision::MGB),
			"sgb" => Ok(HardwareRevision::SGB),
			"sgb2" => Ok(HardwareRevision::SGB2),
			"cgb" => Ok(HardwareRevision::CGB),
			"agb" => Ok(HardwareRevision::AGB),
			"ags" => Ok(HardwareRevision::AGS),
			_ => Err(()),
		}
	}
}

impl TryFrom<char> for HardwareRevision {
	type Error = ();

	fn try_from(value: char) -> Result<Self, Self::Error> {
		use HardwareRevision as R;
		match value {
			'G' => Ok(R::DMG | R::MGB),
			'S' => Ok(R::SGB | R::SGB2),
			'C' => Ok(R::CGB | R::AGB | R::AGS),
			'A' => Ok(R::AGB | R::AGS),
			_ => Err(()),
		}
	}
}

fn extract_compat_flags(path: &str) -> HardwareRevision {
	let name = path.split('/').last().unwrap().split(".gb").next().unwrap();
	let Some(flags_str) = name.split('-').last() else {
		return HardwareRevision::all();
	};
	if flags_str.is_empty() {
		return HardwareRevision::all();
	}
	if flags_str.chars().next().unwrap().is_uppercase() {
		let mut flags = HardwareRevision::empty();
		for c in flags_str.chars() {
			if let Ok(flag) = HardwareRevision::try_from(c) {
				flags |= flag;
			}
		}

		if flags.is_empty() {
			HardwareRevision::all()
		} else {
			flags
		}
	} else if let Ok(flags) = HardwareRevision::try_from(flags_str) {
		flags
	} else {
		HardwareRevision::all()
	}
}

#[test_resources("../test_data/mooneye-test-suite/emulator-only/**/*.gb")]
fn emulator_only(rom: &str) {
	mooneye_test(rom)
}

#[test_resources("../test_data/mooneye-test-suite/acceptance/**/*.gb")]
fn acceptance(rom: &str) {
	mooneye_test(rom)
}

fn mooneye_test(rom: &str) {
	let flags = extract_compat_flags(rom);

	let mut state = if flags.contains(HardwareRevision::CGB) {
		init_emulator_with_rom_cgb(rom)
	} else if flags.contains(HardwareRevision::DMG) {
		init_emulator_with_rom_dmg(rom)
	} else {
		return;
	};

	for _ in 0..1_048_576 * 100 {
		if let Some(Instruction::LD_8(
			ValueRefU8::Reg(CPURegister8::B),
			ValueRefU8::Reg(CPURegister8::B),
		)) = state.step()
		{
			break;
		}
	}

	test_fib_success_code(&state).unwrap();
}
