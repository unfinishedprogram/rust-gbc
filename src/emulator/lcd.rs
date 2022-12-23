use egui::{Color32, ColorImage, Image, TextureHandle};

use crate::app::drawable::Drawable;

pub trait LCDDisplay {
	fn get_size(&self) -> (u8, u8);
	fn put_pixel(&mut self, x: u8, y: u8, color: (u8, u8, u8));
}

#[derive(Clone)]
pub struct LCD {
	buffers: Vec<ColorImage>,
	current_buffer: usize,
	texture: TextureHandle,
	pub scale: f32,
}

impl Drawable for LCD {
	fn draw(&self, ui: &mut egui::Ui) {
		let (x, y) = self.get_size();

		ui.add(Image::new(
			self.texture.id(),
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
	pub fn new(ctx: &egui::Context) -> Self {
		let buffers = vec![
			ColorImage::new([160, 144], Color32::BLACK),
			ColorImage::new([160, 144], Color32::BLACK),
		];
		let texture = ctx.load_texture("LCD", buffers[0].clone(), egui::TextureFilter::Nearest);
		Self {
			scale: 4.0,
			texture,
			current_buffer: 0,
			buffers,
		}
	}

	pub fn swap_buffers(&mut self) {
		self.current_buffer ^= 1;
		self.texture.set(
			self.buffers[self.current_buffer].clone(),
			egui::TextureFilter::Nearest,
		)
	}
}
