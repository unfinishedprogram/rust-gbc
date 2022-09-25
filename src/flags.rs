pub type BitFlagRef = (u16, u8);
use std::borrow::BorrowMut;

use crate::memory::Memory;
pub enum InterruptFlag {
	VBlank = 0,
	LcdStat = 1,
	Timer = 2,
	Serial = 3,
	JoyPad = 4,
}

pub enum LCDFlag {
	BGDisplay = 0,
	OBJDisplayEnable = 1,
	OBJSize = 2,
	BGAndWindowTileDataSelect = 4,
	BGTileMapDisplaySelect = 3,
	WindowDisplayEnable = 5,
	WindowTileMapDisplaySelect = 6,
	LcdDisplayEnable = 7,
}

pub enum TimerFlag {
	Stop = 2,
}

#[repr(u16)]
pub enum BitFlag {
	Interrupt(InterruptFlag) = 0xFF0F,
	LCD(LCDFlag) = 0xFF40,
	Timer(TimerFlag) = 0xFF07,
}

pub fn set_bit_flag(memory: &mut Memory, flag: BitFlag) {
	let mem = memory.borrow_mut();

	let flag = match flag {
		BitFlag::Interrupt(bit) => (0xFF0F, bit as u16),
		BitFlag::LCD(bit) => (0xFF40, bit as u16),
		BitFlag::Timer(bit) => (0xFF07, bit as u16),
	};

	let mask = 1 << flag.1;
	mem[flag.0] |= mask;
}
