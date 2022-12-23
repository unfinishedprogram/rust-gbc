use egui::{ColorImage, Image};

use crate::app::drawable::Drawable;

pub trait LCDDisplay {
	fn get_size(&self) -> (u8, u8);
	fn put_pixel(&mut self, x: u8, y: u8, color: (u8, u8, u8));
	fn get_image_data(&self) -> &Vec<u8>;
}

#[derive(Clone)]
pub struct LCD {
	buffers: (Vec<u8>, Vec<u8>),
	current_buffer: bool,
}

impl Drawable for LCD {
	fn draw(&self, ui: &mut egui::Ui) {
		let (x, y) = self.get_size();

		let buffer = match self.current_buffer {
			true => &self.buffers.0,
			false => &self.buffers.1,
		};

		let texture = ui.ctx().load_texture(
			"LCD",
			ColorImage::from_rgba_unmultiplied([x as usize, y as usize], buffer),
			egui::TextureFilter::Nearest,
		);

		ui.add(Image::new(texture.id(), (x as f32 * 4.0, y as f32 * 4.0)));
	}
}

impl LCDDisplay for LCD {
	fn get_size(&self) -> (u8, u8) {
		(160, 144)
	}

	fn put_pixel(&mut self, x: u8, y: u8, color: (u8, u8, u8)) {
		let (width, height) = self.get_size();
		if x >= width || y >= height {
			return;
		}

		let (r, g, b) = color;
		let index: usize = (y as usize * width as usize + x as usize) * 4;

		let buffer = match self.current_buffer {
			true => &mut self.buffers.1,
			false => &mut self.buffers.0,
		};

		buffer[index] = r;
		buffer[index + 1] = g;
		buffer[index + 2] = b;
	}

	fn get_image_data(&self) -> &Vec<u8> {
		match self.current_buffer {
			true => &self.buffers.0,
			false => &self.buffers.1,
		}
	}
}

impl LCD {
	pub fn new() -> Self {
		Self {
			current_buffer: false,
			buffers: (vec![0xFF; 144 * 160 * 4], vec![0xFF; 144 * 160 * 4]),
		}
	}
	pub fn swap_buffers(&mut self) {
		self.current_buffer = !self.current_buffer;
	}
}

impl Default for LCD {
	fn default() -> Self {
		Self::new()
	}
}
