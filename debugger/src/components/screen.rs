use egui::{load::SizedTexture, Color32, ColorImage, Image, TextureHandle, TextureOptions, Ui};

#[derive(Default)]
pub struct Screen {
	texture: Option<TextureHandle>,
}

impl Screen {
	pub fn draw(&mut self, ui: &mut Ui, buffer: &[u8]) {
		// let image = ColorImage::from_rgba_unmultiplied([160, 144], buffer);
		let buffer = buffer
			.as_chunks()
			.0
			.iter()
			.map(|&[r, g, b, _]| Color32::from_rgb(r, g, b))
			.collect();

		let image = ColorImage {
			size: [160, 144],
			pixels: buffer,
		};

		let texture = self.texture.get_or_insert_with(|| {
			ui.ctx()
				.load_texture("screen", image.clone(), TextureOptions::NEAREST)
		});

		texture.set(image, TextureOptions::NEAREST);

		ui.add(Image::new(SizedTexture::new(
			texture,
			(160.0 * 4.0, 144.0 * 4.0),
		)));
	}
}
