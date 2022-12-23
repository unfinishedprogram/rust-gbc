use std::{
	fs::File,
	io::{BufReader, Read},
};

use crate::emulator::EmulatorState;

fn test_blargg(rom_name: &str) {
	let timeout = 9000000;

	let mut state = EmulatorState::default();

	let rom_handle = File::open(format!("roms/{rom_name}.gb"))
		.unwrap_or_else(|_| panic!("roms/{rom_name}.gb not found"));

	let rom: Vec<u8> = BufReader::new(rom_handle)
		.bytes()
		.map(|byte| byte.unwrap())
		.collect();

	state.load_rom(&rom).unwrap();
	let mut steps_since_last_serial_out: usize = 0;

	let mut last_out_len = 0;

	while steps_since_last_serial_out < timeout {
		state.step();

		steps_since_last_serial_out += 1;
		if state.serial_output.len() != last_out_len {
			last_out_len = state.serial_output.len();
			steps_since_last_serial_out = 0;
			let final_str = std::str::from_utf8(&state.serial_output).unwrap();
			if final_str.contains("Passed") {
				break;
			}
		}
	}

	let final_str = std::str::from_utf8(&state.serial_output).unwrap();

	if !final_str.contains("Passed") {
		if !final_str.contains("Failed") {
			panic!("Timed out without explicit failure, \n{final_str}\n",)
		} else {
			panic!("Failed, \n{final_str}\n",)
		}
	}
}

macro_rules! blarggs_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let rom = $value;
            test_blargg(rom);
        }
    )*
    }
}

blarggs_tests! {
	blarggs_1:"cpu_instrs/01-special",
	blarggs_2:"cpu_instrs/02-interrupts",
	blarggs_3:"cpu_instrs/03-op sp,hl",
	blarggs_4:"cpu_instrs/04-op r,imm",
	blarggs_5:"cpu_instrs/05-op rp",
	blarggs_6:"cpu_instrs/06-ld r,r",
	blarggs_7:"cpu_instrs/07-jr,jp,call,ret,rst",
	blarggs_8:"cpu_instrs/08-misc instrs",
	blarggs_9:"cpu_instrs/09-op r,r",
	blarggs_10:"cpu_instrs/10-bit ops",
	blarggs_11:"cpu_instrs/11-op a,(hl)",

	read_timing:"mem_timing/01-read_timing",
	write_timing:"mem_timing/02-write_timing",
	modify_timing:"mem_timing/03-modify_timing",
	instr_timing:"other/instr_timing",
}
