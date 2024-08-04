use self::colors::{get_closest_color_code, BG_COLOR, FG_COLOR};

pub mod colors;
pub mod cursor;

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

pub fn color_index(r: u8, g: u8, b: u8) -> u8 {
	get_closest_color_code(r, g, b)
}
