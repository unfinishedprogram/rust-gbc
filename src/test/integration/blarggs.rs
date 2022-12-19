use std::{
	fs::File,
	io::{BufReader, Read},
};

use crate::{emulator::EmulatorState, test::mocks::mock_lcd::MockLCD};

fn test_blargg(rom_name: &str, end: usize) {
	let mut lcd = MockLCD::default();
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
		state.step(&mut lcd);

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
	blarggs_1:("cpu_instrs/01-special", 1262731),
	blarggs_2:("cpu_instrs/02-interrupts", 164761),
	blarggs_3:("cpu_instrs/03-op sp,hl", 1070382),
	blarggs_4:("cpu_instrs/04-op r,imm", 1264726),
	blarggs_5:("cpu_instrs/05-op rp", 1765488),
	blarggs_6:("cpu_instrs/06-ld r,r", 245303),
	blarggs_7:("cpu_instrs/07-jr,jp,call,ret,rst", 292712),
	blarggs_8:("cpu_instrs/08-misc instrs", 227427),
	blarggs_9:("cpu_instrs/09-op r,r", 4422293),
	blarggs_10:("cpu_instrs/10-bit ops", 6717390),
	blarggs_11:("cpu_instrs/11-op a,(hl)", 7432135),
}