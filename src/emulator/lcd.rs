use egui::{ColorImage, Image};

use crate::app::drawable::Drawable;

pub trait LCDDisplay {
	fn get_size(&self) -> (u8, u8);
	fn put_pixel(&mut self, x: u8, y: u8, color: (u8, u8, u8));
	fn get_image_data(&self) -> &Vec<u8>;
}

#[derive(Clone)]
pub struct LCD {
	buffer: Vec<u8>,
}

impl Drawable for LCD {
	fn draw(&self, ui: &mut egui::Ui) {
		let (x, y) = self.get_size();

		let texture = ui.ctx().load_texture(
			"LCD",
			ColorImage::from_rgba_unmultiplied([x as usize, y as usize], &self.buffer),
			egui::TextureFilter::Nearest,
		);

		ui.add(Image::new(texture.id(), (x as f32 * 2.0, y as f32 * 2.0)));
	}
}

impl LCDDisplay for LCD {
	fn get_size(&self) -> (u8, u8) {
		(160, 144)
	}

	fn put_pixel(&mut self, x: u8, y: u8, color: (u8, u8, u8)) {
		let (width, height) = self.get_size();

		let x = x % width;
		let y = y % height;

		let (r, g, b) = color;
		let index: usize = (y as usize * width as usize + x as usize) * 4;

		self.buffer[index + 0] = r;
		self.buffer[index + 1] = g;
		self.buffer[index + 2] = b;
	}

	fn get_image_data(&self) -> &Vec<u8> {
		&self.buffer
	}
}

impl LCD {
	pub fn new() -> Self {
		Self {
			buffer: vec![0xFF; 144 * 160 * 4],
		}
	}
}
