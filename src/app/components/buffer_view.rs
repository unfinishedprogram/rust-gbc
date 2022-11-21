use egui::{ColorImage, Image, TextureHandle, Ui, Vec2};

use crate::app::drawable::DrawableMut;

type PixelBuffer = Vec<Vec<[u8; 4]>>;

pub struct BufferView {
	pub pixel_buffer: PixelBuffer,
	size: (usize, usize),
	texture: Option<TextureHandle>,
	name: &'static str,
}

impl DrawableMut for BufferView {
	fn draw(&mut self, ui: &mut Ui) {
		let size = [self.size.0, self.size.1];

		let image =
			ColorImage::from_rgba_unmultiplied(size, to_flat(&self.pixel_buffer).as_slice());

		let texture = self.texture.get_or_insert_with(|| {
			ui.ctx()
				.load_texture(self.name, image.clone(), egui::TextureFilter::Nearest)
		});

		texture.set(image, egui::TextureFilter::Nearest);

		ui.add(Image::new(
			texture.id(),
			Vec2::new((self.size.0 as f32) * 2.0, (self.size.1 as f32) * 2.0),
		));
	}
}

impl BufferView {
	pub fn new(name: &'static str, size: (usize, usize)) -> Self {
		Self {
			name,
			size,
			texture: None,
			pixel_buffer: vec![vec![[255; 4]; size.0]; size.1],
		}
	}
}

fn to_flat(buffer: &PixelBuffer) -> Vec<u8> {
	let mut collector: Vec<u8> = Vec::new();

	for row in buffer {
		for col in row {
			for pixel in col {
				collector.push(pixel.to_owned());
			}
		}
	}

	collector
}
