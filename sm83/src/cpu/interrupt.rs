use crate::bits::*;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
	VBlank = BIT_0,
	LcdStat = BIT_1,
	Timer = BIT_2,
	Serial = BIT_3,
	JoyPad = BIT_4,
}

impl TryFrom<u8> for Interrupt {
	type Error = ();
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			BIT_0 => Ok(Self::VBlank),
			BIT_1 => Ok(Self::LcdStat),
			BIT_2 => Ok(Self::Timer),
			BIT_3 => Ok(Self::Serial),
			BIT_4 => Ok(Self::JoyPad),
			_ => Err(()),
		}
	}
}

impl Interrupt {
	pub fn jump_addr(&self) -> u16 {
		match self {
			Self::VBlank => 0x40,
			Self::LcdStat => 0x48,
			Self::Timer => 0x50,
			Self::Serial => 0x58,
			Self::JoyPad => 0x60,
		}
	}

	pub fn flag_bit(&self) -> u8 {
		*self as u8
	}
}
