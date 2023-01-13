use crate::util::bits::*;

pub struct TileData(pub u16, pub Option<TileAttributes>);

pub struct TileAttributes {
	pub vertical_flip: bool,
	pub horizontal_flip: bool,
	pub v_ram_bank: usize,
	pub bg_priority: bool,
	pub palette_number: usize,
}

impl TileAttributes {
	pub fn new(attributes: u8) -> TileAttributes {
		TileAttributes {
			bg_priority: attributes & BIT_7 == BIT_7,
			vertical_flip: attributes & BIT_6 == BIT_6,
			horizontal_flip: attributes & BIT_5 == BIT_5,
			v_ram_bank: ((attributes >> 3) & 1) as usize,
			palette_number: (attributes & 0b111) as usize,
		}
	}
}
