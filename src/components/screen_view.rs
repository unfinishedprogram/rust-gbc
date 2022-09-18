use egui::{ColorImage, Context, Image, TextureHandle, Vec2};

pub struct ScreenViewState {
	pub pixel_buffer: [u8; 160 * 144 * 4],
	size: (usize, usize),
	texture: Option<TextureHandle>,
	name: &'static str,
}

impl Default for ScreenViewState {
	fn default() -> Self {
		Self {
			name: "Screen",
			texture: None,
			size: (160, 144),
			pixel_buffer: [255; 160 * 144 * 4],
		}
	}
}

impl ScreenViewState {
	pub fn new(name: &'static str) -> Self {
		Self {
			name,
			texture: None,
			size: (160, 144),
			pixel_buffer: [255; 160 * 144 * 4],
		}
	}

	pub fn set_buffer(&mut self, buffer: &[u8]) {
		if buffer.len() > self.pixel_buffer.len() {
			panic!("Size mismatch between buffers");
		}
		for i in 0..buffer.len() {
			self.pixel_buffer[i] = buffer[i];
		}
	}
}

pub fn screen_view(ctx: &Context, state: &mut ScreenViewState) {
	let image =
		ColorImage::from_rgba_unmultiplied([state.size.0, state.size.1], &state.pixel_buffer);

	match &mut state.texture {
		None => {
			_ = state.texture.insert(ctx.load_texture(
				"screen-texture",
				image,
				egui::TextureFilter::Nearest,
			));
		}
		Some(texture) => {
			texture.set(image, egui::TextureFilter::Nearest);
			egui::Window::new(state.name)
				.resizable(false)
				.show(ctx, |ui| {
					ui.add(Image::new(
						texture.id(),
						Vec2::new((state.size.0 as f32) * 2.0, (state.size.1 as f32) * 2.0),
					))
				});
		}
	}
}
