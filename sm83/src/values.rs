use std::fmt;

use crate::registers::{CPURegister16, CPURegister8};

impl From<CPURegister16> for ValueRefU16 {
	fn from(value: CPURegister16) -> Self {
		Self::Reg(value)
	}
}

impl From<CPURegister16> for ValueRefU8 {
	fn from(value: CPURegister16) -> Self {
		Self::Mem(value.into())
	}
}

impl From<CPURegister8> for ValueRefU8 {
	fn from(value: CPURegister8) -> Self {
		Self::Reg(value)
	}
}

impl From<u8> for ValueRefU8 {
	fn from(value: u8) -> Self {
		Self::Raw(value)
	}
}

impl From<i8> for ValueRefI8 {
	fn from(value: i8) -> Self {
		Self(value)
	}
}

impl From<u16> for ValueRefU16 {
	fn from(value: u16) -> Self {
		Self::Raw(value)
	}
}

#[derive(Clone, Copy)]
pub enum ValueRefU8 {
	Reg(CPURegister8),
	Mem(ValueRefU16),
	Raw(u8),
	MemOffsetRaw(u8),
	MemOffsetReg(CPURegister8),
}

#[derive(Clone, Copy)]
pub enum ValueRefU16 {
	Reg(CPURegister16),
	Mem(u16),
	Raw(u16),
}

#[derive(Clone, Copy)]
pub struct ValueRefI8(pub i8);

impl fmt::Debug for ValueRefU16 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use ValueRefU16::*;
		match self {
			Raw(x) => write!(f, "${x:04X}"),
			Mem(x) => write!(f, "[${}]", format_memref(*x)),
			Reg(x) => write!(f, "{x:?}"),
		}
	}
}

impl fmt::Debug for ValueRefU8 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use ValueRefU8::*;
		match self {
			Raw(x) => write!(f, "${x:02X}"),
			Mem(x) => write!(f, "[{x:?}]"),
			Reg(x) => write!(f, "{x:?}"),
			MemOffsetRaw(offset) => write!(f, "[${}]", format_memref((*offset as u16) + 0xFF00)),
			MemOffsetReg(reg) => write!(f, "[{reg:?}]"),
		}
	}
}

impl fmt::Debug for ValueRefI8 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let x = self.0;
		if x >= 0 {
			write!(f, "${x:02X}")
		} else {
			write!(f, "-${:02X}", x.unsigned_abs())
		}
	}
}

fn format_memref(addr: u16) -> String {
	match addr {
		0xFF04 => "DIV".to_owned(),
		0xFF05 => "TIMA".to_owned(),
		0xFF06 => "TMA".to_owned(),
		0xFF07 => "TAC".to_owned(),
		0xFF10 => "NR10".to_owned(),
		0xFF11 => "NR11".to_owned(),
		0xFF12 => "NR12".to_owned(),
		0xFF14 => "NR14".to_owned(),
		0xFF16 => "NR21".to_owned(),
		0xFF17 => "NR22".to_owned(),
		0xFF19 => "NR24".to_owned(),
		0xFF1A => "NR30".to_owned(),
		0xFF1B => "NR31".to_owned(),
		0xFF1C => "NR32".to_owned(),
		0xFF1E => "NR33".to_owned(),
		0xFF20 => "NR41".to_owned(),
		0xFF21 => "NR42".to_owned(),
		0xFF22 => "NR43".to_owned(),
		0xFF23 => "NR44".to_owned(),
		0xFF24 => "NR50".to_owned(),
		0xFF25 => "NR51".to_owned(),
		0xFF26 => "NR52".to_owned(),
		0xFF40 => "LCDC".to_owned(),
		0xFF41 => "STAT".to_owned(),
		0xFF42 => "SCY".to_owned(),
		0xFF43 => "SCX".to_owned(),
		0xFF44 => "LY".to_owned(),
		0xFF45 => "LYC".to_owned(),
		0xFF46 => "DMA".to_owned(),
		0xFF47 => "BGP".to_owned(),
		0xFF48 => "OBP0".to_owned(),
		0xFF49 => "OBP1".to_owned(),
		0xFF4A => "WY".to_owned(),
		0xFF4B => "WX".to_owned(),
		0xFF01 => "SB".to_owned(),
		0xFF02 => "SC".to_owned(),
		0xFF0F => "IF".to_owned(),
		addr => format!("{:04X}", addr),
	}
}
