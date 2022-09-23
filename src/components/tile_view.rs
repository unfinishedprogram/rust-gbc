use egui::{ColorImage, Context, Image, TextureHandle, Vec2};

pub struct TileViewState {
	pub pixel_buffer: [[[u8; 4]; 256]; 256],
	size: (usize, usize),
	texture: Option<TextureHandle>,
	name: &'static str,
}

impl Default for TileViewState {
	fn default() -> Self {
		Self {
			name: "Tile View",
			texture: None,
			size: (256, 256),
			pixel_buffer: [[[255; 4]; 256]; 256],
		}
	}
}

impl TileViewState {
	pub fn set_buffer(&mut self, buffer: &[[[u8; 4]; 256]; 256]) {
		if buffer.len() > self.pixel_buffer.len() {
			panic!("Size mismatch between buffers");
		}
		for i in 0..buffer.len() {
			self.pixel_buffer[i] = buffer[i];
		}
	}
}

fn to_flat(buffer: &[[[u8; 4]; 256]; 256]) -> &[u8] {
	let flat = buffer.flatten();
	return flat.flatten();
}

pub fn tile_view(ctx: &Context, state: &mut TileViewState) {
	let image = ColorImage::from_rgba_unmultiplied(
		[state.size.0, state.size.1],
		to_flat(&state.pixel_buffer),
	);

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
