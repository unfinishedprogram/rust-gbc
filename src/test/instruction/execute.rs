use crate::{
	emulator::{
		cpu::{
			flags::*,
			instruction::{execute::execute_instruction, Instruction},
			registers::CPURegister16,
			values::{ValueRefU16, ValueRefU8},
			CPU,
		},
		EmulatorState,
	},
	test::{instruction::logger::log_execute, mocks::mock_rom::create_rom},
};

use std::{
	fs::File,
	io::{self, BufRead, Read},
	vec,
};

fn get_state(data: Vec<u8>) -> EmulatorState {
	let mut state = EmulatorState::default().init();
	state.load_rom(&create_rom(data));
	state
}

#[test]
fn initial_state() {
	use CPURegister16::*;
	use ValueRefU16::{Raw, Reg};
	use ValueRefU8::Mem;

	let mut state = get_state(vec![1, 2, 3, 4]);

	// Registers
	assert_eq!(state.read_16(Reg(AF)), 0x01B0);
	assert_eq!(state.read_16(Reg(BC)), 0x0013);
	assert_eq!(state.read_16(Reg(DE)), 0x00D8);
	assert_eq!(state.read_16(Reg(HL)), 0x014D);
	assert_eq!(state.read_16(Reg(SP)), 0xFFFE);

	assert_eq!(state.read_8(&Mem(Raw(0x0100))), 1);
	assert_eq!(state.read_8(&Mem(Raw(0x0101))), 2);
	assert_eq!(state.read_8(&Mem(Raw(0x0102))), 3);
	assert_eq!(state.read_8(&Mem(Raw(0x0103))), 4);
	assert_eq!(state.read_8(&Mem(Raw(0x0104))), 0);
}

fn assert_flags(state: &EmulatorState, flags: (bool, bool, bool, bool)) {
	assert_eq!(state.get_flag(Flag::Z), flags.0);
	assert_eq!(state.get_flag(Flag::N), flags.1);
	assert_eq!(state.get_flag(Flag::H), flags.2);
	assert_eq!(state.get_flag(Flag::C), flags.3);
}

#[test]
fn flags() {
	use Flag::*;
	let mut state = get_state(vec![0]);

	assert_flags(&state, (true, false, true, true));

	state.clear_flag(C);
	state.clear_flag(H);
	state.clear_flag(N);
	state.clear_flag(Z);

	assert_flags(&state, (false, false, false, false));

	state.set_flag(C);
	state.set_flag(H);
	state.set_flag(N);
	state.set_flag(Z);

	assert_flags(&state, (true, true, true, true));
}

#[test]
fn load_16() {
	use CPURegister16::*;
	use Instruction::*;
	use ValueRefU16::*;

	let mut state = get_state(vec![1, 2, 3, 4]);
	for i in 0..0xFFFFu16 {
		execute_instruction(LD_16(Reg(AF), Raw(i)), &mut state);
		assert_eq!(state.read_16(Reg(AF)), i & 0xFFF0);

		execute_instruction(LD_16(Reg(BC), Raw(i)), &mut state);
		assert_eq!(state.read_16(Reg(BC)), i);

		execute_instruction(LD_16(Reg(DE), Raw(i)), &mut state);
		assert_eq!(state.read_16(Reg(DE)), i);

		execute_instruction(LD_16(Reg(HL), Raw(i)), &mut state);
		assert_eq!(state.read_16(Reg(HL)), i);

		execute_instruction(LD_16(Reg(SP), Raw(i)), &mut state);
		assert_eq!(state.read_16(Reg(SP)), i);
	}
}

// #[test]
// fn dec_8() {
// 	let mut state = get_state(vec![0]);
// 	use CPURegister8::*;
// 	use Instruction::*;
// 	use ValueRefU8::*;

// 	state.clear_flag(Flag::C);
// 	state.clear_flag(Flag::H);
// 	state.clear_flag(Flag::N);
// 	state.clear_flag(Flag::Z);

// 	state.write_8(&Reg(A), 0);
// 	execute_instruction(DEC_8(Reg(A)), &mut state);
// 	assert_flags(&state, (false, true, true, false));
// }

// #[test]
// fn tetris() {
// 	let mut state = EmulatorState::default().init();
// 	let tetris_handle = File::open("roms/06-ld r,r.gb").unwrap();
// 	let mut rom = vec![];
// 	_ = io::BufReader::new(tetris_handle).read_to_end(&mut rom);
// 	println!("{}", rom.len());
// 	state.load_rom(&rom);

// 	let handle = File::open("logs/test.log").unwrap();
// 	let lines = io::BufReader::new(handle).lines();

// 	let mut last: String = "".to_string();
// 	for line in lines {
// 		let exec = log_execute(&mut state);
// 		let line = line.unwrap();
// 		if exec != line {
// 			println!("{last}");
// 			assert_eq!(exec, line);
// 		}
// 		last = line;
// 		// println!("{:}", state.ppu_state.cycle / 2);
// 	}
// }

fn check_parity(rom_name: &str) {
	let mut state = EmulatorState::default().init();
	let rom_handle = File::open(format!("roms/{rom_name}.gb")).unwrap();

	let mut rom = vec![];
	_ = io::BufReader::new(rom_handle).read_to_end(&mut rom);
	state.load_rom(&rom);

	let log_handle = File::open(format!("logs/{rom_name}.log")).unwrap();
	let lines = io::BufReader::new(log_handle).lines();

	let mut last: String = "".to_string();
	for line in lines {
		let exec = log_execute(&mut state);
		let line = line.unwrap();
		if exec != line {
			println!("Failed : {rom_name}");
			println!("{last}");
			assert_eq!(exec, line);
		}
		last = line;
	}
}

fn test_blargg(rom_name: &str, end: usize) {
	let mut state = EmulatorState::default().init();
	let rom_handle = File::open(format!("roms/{rom_name}.gb")).unwrap();

	let mut rom = vec![];
	_ = io::BufReader::new(rom_handle).read_to_end(&mut rom);
	state.load_rom(&rom);

	let mut last: usize = 0;
	let mut left = end;
	let mut last_write = 0;
	while left > 0 {
		left -= 1;
		_ = log_execute(&mut state);
		if state.serial_output.len() != last {
			last = state.serial_output.len();
			last_write = end - left;
		}
	}

	let final_str = std::str::from_utf8(&state.serial_output).unwrap();
	if last_write != end {
		println!("!Test took more cycles than needed. Last Write at: {last_write}")
	}
	assert!(final_str.contains("Passed"))
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
	blarggs_1:("01-special", 1277946),
	blarggs_2:("02-interrupts", 210428),
	blarggs_3:("03-op sp,hl", 1089986),
	blarggs_4:("04-op r,imm", 1075092),
	blarggs_5:("05-op rp", 1789474),
	blarggs_6:("06-ld r,r", 269289),
	blarggs_7:("07-jr,jp,call,ret,rst", 321087),
	blarggs_8:("08-misc instrs", 251413),
	blarggs_9:("09-op r,r", 1057765),
	blarggs_10:("10-bit ops", 6732598),
	blarggs_11:("11-op a,(hl)", 3812054),
}
