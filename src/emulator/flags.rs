pub type BitFlagRef = (u16, u8);

use super::memory_mapper::MemoryMapper;
use crate::util::bits::*;

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
	InterruptEnable = 0xFFFF,
	InterruptRequest = 0xFF0F,
	JoyPad = 0xFF00,
	LCD = 0xFF40,
	Stat = 0xFF41,
	Timer = 0xFF07,
}

fn flag_to_tuple(flag: BitFlag, bit: u8) -> BitFlagRef {
	(flag as u16, bit)
}

pub fn get_bit_flag(mem: &dyn MemoryMapper, flag: BitFlag, bit: u8) -> bool {
	let (addr, bit) = flag_to_tuple(flag, bit);
	mem.read(addr) & bit == bit
}

pub fn clear_bit_flag(mem: &mut dyn MemoryMapper, flag: BitFlag, bit: u8) {
	let (addr, bit) = flag_to_tuple(flag, bit);
	mem.write(addr, mem.read(addr) & (!bit));
}

pub fn set_bit_flag(mem: &mut dyn MemoryMapper, flag: BitFlag, bit: u8) {
	let (addr, bit) = flag_to_tuple(flag, bit);
	mem.write(addr, mem.read(addr) | bit);
}

pub fn set_bit_flag_to(mem: &mut dyn MemoryMapper, flag: BitFlag, bit: u8, status: bool) {
	if status {
		set_bit_flag(mem, flag, bit)
	} else {
		clear_bit_flag(mem, flag, bit)
	}
}
