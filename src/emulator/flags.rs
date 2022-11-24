pub type BitFlagRef = (u16, u8);

use super::memory_mapper::MemoryMapper;
use crate::util::{
	bit_ops::{clear_bit_mask, get_bit, set_bit_mask},
	bits::*,
};

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum InterruptFlag {
	VBlank = BIT_0,
	LcdStat = BIT_1,
	Timer = BIT_2,
	Serial = BIT_3,
	JoyPad = BIT_4,
}

#[repr(u8)]
pub enum STATFlag {
	LYCeqLY = BIT_2,
	HBlankStatInterruptEnable = BIT_3,
	VBlankStatInterruptEnable = BIT_4,
	OAMStatInterruptEnable = BIT_5,
	LYCeqLUInterruptEnable = BIT_6,
}

#[repr(u8)]
pub enum JoyPadFlag {
	RightOrA = BIT_0,
	LeftOrB = BIT_1,
	UpOrSelect = BIT_2,
	DownOrStart = BIT_3,
	SelectDirectionButtons = BIT_4,
	SelectActionButtons = BIT_5,
}

#[repr(u8)]
pub enum LCDFlag {
	BGDisplay = BIT_0,
	OBJDisplayEnable = BIT_1,
	OBJSize = BIT_2,
	BGTileMapDisplaySelect = BIT_3,
	BGAndWindowTileDataSelect = BIT_4,
	WindowDisplayEnable = BIT_5,
	WindowTileMapDisplaySelect = BIT_6,
	LcdDisplayEnable = BIT_7,
}

#[repr(u8)]
pub enum TimerFlag {
	Stop = BIT_2,
}

#[repr(u16)]
pub enum BitFlag {
	InterruptEnable(InterruptFlag) = 0xFFFF,
	InterruptRequest(InterruptFlag) = 0xFF0F,
	JoyPad(JoyPadFlag) = 0xFF00,
	LCD(LCDFlag) = 0xFF40,
	Stat(STATFlag) = 0xFF41,
	Timer(TimerFlag) = 0xFF07,
}

fn flag_to_tuple(flag: BitFlag) -> BitFlagRef {
	match flag {
		BitFlag::InterruptEnable(bit) => (0xFFFF, bit as u8),
		BitFlag::InterruptRequest(bit) => (0xFF0F, bit as u8),
		BitFlag::LCD(bit) => (0xFF40, bit as u8),
		BitFlag::Stat(bit) => (0xFF41, bit as u8),
		BitFlag::Timer(bit) => (0xFF07, bit as u8),
		BitFlag::JoyPad(bit) => (0xFF00, bit as u8),
	}
}

pub fn get_bit_flag(mem: &dyn MemoryMapper, flag: BitFlag) -> bool {
	let (addr, bit) = flag_to_tuple(flag);
	mem.read(addr) & bit == bit
}

pub fn clear_bit_flag(mem: &mut dyn MemoryMapper, flag: BitFlag) {
	let (addr, bit) = flag_to_tuple(flag);
	mem.write(addr, mem.read(addr) & (!bit));
}

pub fn set_bit_flag(mem: &mut dyn MemoryMapper, flag: BitFlag) {
	let (addr, bit) = flag_to_tuple(flag);
	mem.write(addr, mem.read(addr) | bit);
}

pub fn set_bit_flag_to(mem: &mut dyn MemoryMapper, flag: BitFlag, status: bool) {
	if status {
		set_bit_flag(mem, flag)
	} else {
		clear_bit_flag(mem, flag)
	}
}
