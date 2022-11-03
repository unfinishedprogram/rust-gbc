use std::{assert_matches::assert_matches, vec};

use crate::emulator::{
	cpu::{
		instruction::{fetch::fetch_instruction, Instruction},
		registers::{CPURegister16, CPURegister8},
		values::{ValueRefU16, ValueRefU8},
	},
	memory_mapper::MemoryMapper,
};

use crate::test::mocks::mock_emulator::MockEmulator;

fn parse_bytes(bytes: Vec<u8>) -> Instruction {
	let mut state = MockEmulator::default();

	for (index, byte) in bytes.into_iter().enumerate() {
		state.write((index + 0x100) as u16, byte)
	}

	fetch_instruction(&mut state)
}

#[test]
fn misc_ctrl() {
	use Instruction::*;

	assert_matches!(parse_bytes(vec![0x00]), NOP);
	assert_matches!(parse_bytes(vec![0x10]), STOP);
	assert_matches!(parse_bytes(vec![0x76]), HALT);
	assert_matches!(parse_bytes(vec![0xF3]), DI);
	assert_matches!(parse_bytes(vec![0xFB]), EI);
}

#[test]

fn move_load_16_bit() {
	use CPURegister16::*;
	use Instruction::*;
	use ValueRefU16::*;

	let pb = parse_bytes;

	assert_eq!(u16::from_le_bytes([0xFF, 0]), 0x00FF);
	assert_eq!(u16::from_le_bytes([0, 0xFF]), 0xFF00);

	assert_matches!(pb(vec![0x01, 0x34, 0x12]), LD_16(Reg(BC), Raw(0x1234)));
	assert_matches!(pb(vec![0x11, 0x34, 0x12]), LD_16(Reg(DE), Raw(0x1234)));
	assert_matches!(pb(vec![0x21, 0x34, 0x12]), LD_16(Reg(HL), Raw(0x1234)));
	assert_matches!(pb(vec![0x31, 0x34, 0x12]), LD_16(Reg(SP), Raw(0x1234)));
}

#[test]
fn opcode_gaps() {
	// These should all be errors, which act as no-ops
	use Instruction::*;
	assert_matches!(parse_bytes(vec![0xD3]), ERROR(0xD3));
	assert_matches!(parse_bytes(vec![0xE3]), ERROR(0xE3));
	assert_matches!(parse_bytes(vec![0xF4]), ERROR(0xF4));
	assert_matches!(parse_bytes(vec![0xE4]), ERROR(0xE4));
	assert_matches!(parse_bytes(vec![0xDB]), ERROR(0xDB));
	assert_matches!(parse_bytes(vec![0xEB]), ERROR(0xEB));
	assert_matches!(parse_bytes(vec![0xEC]), ERROR(0xEC));
	assert_matches!(parse_bytes(vec![0xFC]), ERROR(0xFC));
	assert_matches!(parse_bytes(vec![0xDD]), ERROR(0xDD));
	assert_matches!(parse_bytes(vec![0xED]), ERROR(0xED));
	assert_matches!(parse_bytes(vec![0xFD]), ERROR(0xFD));
}

#[test]
fn move_load_8_bit() {
	use CPURegister8::*;
	use Instruction::*;
	use ValueRefU8::*;

	let pb = parse_bytes;

	assert_matches!(
		pb(vec![0xFA, 0x00]),
		LD_8(Reg(A), Mem(ValueRefU16::Raw(0x00)))
	);
}
