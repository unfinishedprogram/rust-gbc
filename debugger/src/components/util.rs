use lazy_static::lazy_static;

lazy_static! {
	static ref U8_LOOKUP: Vec<String> = (0..=0xFF).map(|i| format!("{i:02X}")).collect();
	static ref U16_LOOKUP: Vec<String> = (0..=0xFFFF).map(|i| format!("{i:04X}")).collect();
}

pub fn hex_str_u8(v: u8) -> &'static str {
	U8_LOOKUP[v as usize].as_str()
}
pub fn hex_str_u16(v: u16) -> &'static str {
	U16_LOOKUP[v as usize].as_str()
}
