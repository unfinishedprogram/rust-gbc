pub type BitFlagRef = (u16, u8);

use crate::memory::Memory;

#[derive(Copy, Clone)]
pub enum InterruptFlag {
	VBlank = 0,
	LcdStat = 1,
	Timer = 2,
	Serial = 3,
	JoyPad = 4,
}

pub enum STATFlag {
	LYCeqLU = 2,
	HBlankStatInterruptEnable = 3,
	VBlankStatInterruptEnable = 4,
	OAMStatInterruptEnable = 5,
	LYCeqLUInterruptEnable = 6,
}

pub enum JoyPadFlag {
	RightOrA = 0,
	LeftOrB = 1,
	UpOrSelect = 2,
	DownOrStart = 3,
	SelectDirectionButtons = 4,
	SelectActionButtons = 5,
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
	InterruptEnable(InterruptFlag) = 0xFFFF,
	InterruptRequest(InterruptFlag) = 0xFF0F,
	JoyPad(JoyPadFlag) = 0xFF00,
	LCD(LCDFlag) = 0xFF40,
	Stat(STATFlag) = 0xFF41,
	Timer(TimerFlag) = 0xFF07,
}

fn flag_to_tuple(flag: BitFlag) -> (u16, u16) {
	match flag {
		BitFlag::InterruptEnable(bit) => (0xFFFF, bit as u16),
		BitFlag::InterruptRequest(bit) => (0xFF0F, bit as u16),
		BitFlag::LCD(bit) => (0xFF40, bit as u16),
		BitFlag::Stat(bit) => (0xFF41, bit as u16),
		BitFlag::Timer(bit) => (0xFF07, bit as u16),
		BitFlag::JoyPad(bit) => (0xFF00, bit as u16),
	}
}

pub fn get_bit_flag(mem: &Memory, flag: BitFlag) -> bool {
	let flag = flag_to_tuple(flag);
	return (mem[flag.0] >> flag.1) & 1 == 1;
}

pub fn clear_bit_flag(mem: &mut Memory, flag: BitFlag) {
	let flag = flag_to_tuple(flag);
	let mask = !(1 << flag.1);
	mem[flag.0] &= mask;
}

pub fn set_bit_flag(mem: &mut Memory, flag: BitFlag) {
	let flag = flag_to_tuple(flag);
	let mask = 1 << flag.1;
	mem[flag.0] |= mask;
}

pub fn set_bit_flag_to(mem: &mut Memory, flag: BitFlag, status: bool) {
	if status {
		set_bit_flag(mem, flag)
	} else {
		clear_bit_flag(mem, flag)
	}
}
