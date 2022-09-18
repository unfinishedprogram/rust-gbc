pub fn bit_set(value: u8, bit: u8) -> bool {
	match bit {
		0..=7 => (value >> bit) & 1 == 1,
		_ => false,
	}
}
