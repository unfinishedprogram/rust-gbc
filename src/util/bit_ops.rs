pub fn get_bit(byte: u8, bit: u8) -> bool {
	(byte >> (7 - bit)) & 1 == 1
}

pub fn clear_bit_mask(bit: u8) -> u8 {
	!(1 << (7 - bit))
}

pub fn set_bit_mask(bit: u8) -> u8 {
	1 << (7 - bit)
}
