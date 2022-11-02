use crate::{
	emulator::{
		cpu::{
			flags::*,
			instruction::{
				condition::Condition, execute::execute_instruction, fetch::fetch_instruction,
				Instruction,
			},
			registers::{CPURegister16, CPURegister8},
			values::{ValueRefI8, ValueRefU16, ValueRefU8},
			CPU,
		},
		memory_mapper::MemoryMapper,
		EmulatorState,
	},
	test::{instruction::logger::log_execute, mocks::mock_rom::create_rom},
};
use std::io::{self, BufRead};
use std::path::Path;
use std::{assert_matches::assert_matches, vec};
use std::{fs::File, io::Read};

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

	assert_eq!(state.read_8(Mem(Raw(0x0100))), 1);
	assert_eq!(state.read_8(Mem(Raw(0x0101))), 2);
	assert_eq!(state.read_8(Mem(Raw(0x0102))), 3);
	assert_eq!(state.read_8(Mem(Raw(0x0103))), 4);
	assert_eq!(state.read_8(Mem(Raw(0x0104))), 0);
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
		assert_eq!(state.read_16(Reg(AF)), i);

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

fn dec_8() {
	let mut state = get_state(vec![0]);
	use CPURegister8::*;
	use Instruction::*;
	use ValueRefU8::*;

	state.clear_flag(Flag::C);
	state.clear_flag(Flag::H);
	state.clear_flag(Flag::N);
	state.clear_flag(Flag::Z);

	state.write_8(Reg(A), 0);
	execute_instruction(DEC_8(Reg(A)), &mut state);
	assert_flags(&state, (false, true, true, false));
}

#[test]
fn tetris() {
	let mut state = EmulatorState::default().init();
	let tetris_handle = File::open("roms/tetris.gb").unwrap();
	let mut rom = vec![];
	_ = io::BufReader::new(tetris_handle).read_to_end(&mut rom);
	println!("{}", rom.len());
	state.load_rom(&rom);

	let handle = File::open("logs/Tetris (World).log").unwrap();
	let lines = io::BufReader::new(handle).lines();

	for line in lines {
		println!("{}", state.cpu_state.registers.pc);
		assert_eq!(log_execute(&mut state), line.unwrap());
	}
}
