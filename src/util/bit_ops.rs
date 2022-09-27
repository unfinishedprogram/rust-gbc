pub fn get_bit(byte: &u8, bit: u8) -> bool {
	return (byte >> (7 - bit)) & 1 == 1;
}

pub fn clear_bit(byte: &mut u8, bit: u8) {
	let mask = !(1 << (7 - bit));
	*byte &= mask;
}

pub fn set_bit(byte: &mut u8, bit: u8) {
	let mask = 1 << (7 - bit);
	*byte |= mask;
}
