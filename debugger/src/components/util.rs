use lazy_static::lazy_static;

lazy_static! {
	static ref U8_LOOKUP: Vec<String> = (0..=0xFF).map(|i| format!("{:02X}", i)).collect();
	static ref U16_LOOKUP: Vec<String> = (0..=0xFFFF).map(|i| format!("{:04X}", i)).collect();
}

pub fn hex_str_u8(v: u8) -> &'static str {
	U8_LOOKUP[v as usize].as_str()
}
pub fn hex_str_u16(v: u16) -> &'static str {
	U16_LOOKUP[v as usize].as_str()
}
