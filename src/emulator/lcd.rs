use egui::{Color32, ColorImage, Image, TextureHandle};
use serde::Serialize;

use crate::app::drawable::DrawableMut;

pub trait LCDDisplay {
	fn get_size(&self) -> (u8, u8);
	fn put_pixel(&mut self, x: u8, y: u8, color: (u8, u8, u8));
}

#[derive(Clone, Serialize)]
pub struct LCD {
	buffers: Vec<ColorImage>,
	current_buffer: usize,
	pub scale: f32,
}

impl DrawableMut for LCD {
	fn draw(&mut self, ui: &mut egui::Ui) {
		let (x, y) = self.get_size();
		ui.add(Image::new(
			ui.ctx()
				.load_texture(
					"LCD",
					self.buffers[self.current_buffer].clone(),
					egui::TextureFilter::Nearest,
				)
				.id(),
			(x as f32 * self.scale, y as f32 * self.scale),
		));
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

		let index: usize = y as usize * width as usize + x as usize;

		let image = &mut self.buffers[self.current_buffer];
		image.pixels[index] = Color32::from_rgb(r, g, b);
	}
}

impl LCD {
	pub fn new() -> Self {
		let buffers = vec![
			ColorImage::new([160, 144], Color32::BLACK),
			ColorImage::new([160, 144], Color32::BLACK),
		];
		Self {
			scale: 3.0,
			current_buffer: 0,
			buffers,
		}
	}

	pub fn swap_buffers(&mut self) {
		self.current_buffer ^= 1;
	}

	pub fn get_current_as_bytes(&self) -> Vec<u8> {
		let mut res: Vec<u8> = vec![];

		for pixel in &self.buffers[self.current_buffer].pixels {
			res.push(pixel.r());
			res.push(pixel.g());
			res.push(pixel.b());
		}

		res
	}
}

impl Default for LCD {
	fn default() -> Self {
		Self::new()
	}
}
