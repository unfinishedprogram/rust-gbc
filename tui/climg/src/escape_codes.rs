use lazy_static::lazy_static;

pub mod cursor;

lazy_static! {
	pub static ref FG_COLOR: [String; 256] = (0..256)
		.map(|i| format!("{ESC}38;5;{i:}m"))
		.collect::<Vec<String>>()
		.try_into()
		.unwrap();
	pub static ref BG_COLOR: [String; 256] = (0..256)
		.map(|i| format!("{ESC}[48;5;{i:}m"))
		.collect::<Vec<String>>()
		.try_into()
		.unwrap();
	pub static ref NUM_CHAR: [String; 256] = (0..256)
		.map(|i| format!("{i}"))
		.collect::<Vec<String>>()
		.try_into()
		.unwrap();
}

type RGBColor = (u8, u8, u8);
pub static ESC: &str = "\u{1b}[";

pub trait EscapeCodeSupplier {
	fn fg(&self, color: RGBColor) -> String;
	fn bg(&self, color: RGBColor) -> String;
}

pub struct EscapeCodes;
impl EscapeCodeSupplier for EscapeCodes {
	fn fg(&self, (r, g, b): RGBColor) -> String {
		format!("{ESC}38;2;{r};{g};{b}m")
	}

	fn bg(&self, (r, g, b): RGBColor) -> String {
		format!("{ESC}48;2;{r};{g};{b}m")
	}
}

pub fn bg(index: u8) -> &'static str {
	&BG_COLOR[index as usize]
}

pub fn fg(index: u8) -> &'static str {
	&FG_COLOR[index as usize]
}
