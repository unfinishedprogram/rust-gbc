use crate::emulator::lcd::LCDDisplay;

#[derive(Default)]
pub struct MockLCD {
	img_dat: Vec<u8>,
}

impl LCDDisplay for MockLCD {
	fn get_size(&self) -> (u8, u8) {
		(160, 144)
	}
	fn put_pixel(&mut self, _x: u8, _y: u8, _color: (u8, u8, u8)) {}
	fn get_image_data(&self) -> &Vec<u8> {
		&self.img_dat
	}
}
