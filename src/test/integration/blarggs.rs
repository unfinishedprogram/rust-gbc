use std::{
	fs::File,
	io::{BufReader, Read},
};

use crate::emulator::EmulatorState;

fn test_blargg(rom_name: &str, end: usize) {
	let mut state = EmulatorState::default();

	let rom_handle = File::open(format!("roms/{rom_name}.gb"))
		.unwrap_or_else(|_| panic!("roms/{rom_name}.gb not found"));

	let rom: Vec<u8> = BufReader::new(rom_handle)
		.bytes()
		.map(|byte| byte.unwrap())
		.collect();

	state.load_rom(&rom).unwrap();

	let mut last = 0;
	let mut left = end;
	let mut last_write = 0;
	while left > 0 {
		left -= 1;
		state.step();

		if state.serial_output.len() != last {
			last = state.serial_output.len();
			last_write = end - left;
		}
	}

	let final_str = std::str::from_utf8(&state.serial_output).unwrap();

	if last_write != end {
		panic!("!Test took more cycles than needed. Last Write at: {last_write}")
	}

	if !final_str.contains("Passed") {
		panic!("\n{final_str}\n",)
	}
}

macro_rules! blarggs_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (rom, end) = $value;
            test_blargg(rom, end);
        }
    )*
    }
}

blarggs_tests! {
	blarggs_1:("cpu_instrs/01-special", 1283844),
	blarggs_2:("cpu_instrs/02-interrupts", 182072),
	blarggs_3:("cpu_instrs/03-op sp,hl", 1089349),
	blarggs_4:("cpu_instrs/04-op r,imm", 1278464),
	blarggs_5:("cpu_instrs/05-op rp", 1788445),
	blarggs_6:("cpu_instrs/06-ld r,r", 263304),
	blarggs_7:("cpu_instrs/07-jr,jp,call,ret,rst", 315064),
	blarggs_8:("cpu_instrs/08-misc instrs", 239583),
	blarggs_9:("cpu_instrs/09-op r,r", 4442282),
	blarggs_10:("cpu_instrs/10-bit ops", 6739714),
	blarggs_11:("cpu_instrs/11-op a,(hl)", 7452100),



	read_timing:("mem_timing/01-read_timing", 189183),
	write_timing:("mem_timing/02-write_timing", 190143),
	modify_timing:("mem_timing/03-modify_timing", 219951),


	instr_timing:("other/instr_timing", 267265),
}
