use egui::{ColorImage, Context, Image, TextureHandle, Vec2};

pub struct ScreenViewState {
	pixel_buffer: [u8; 160 * 144 * 4],
	texture: Option<TextureHandle>,
}

impl Default for ScreenViewState {
	fn default() -> Self {
		Self {
			pixel_buffer: [255; 160 * 144 * 4],
			texture: None,
		}
	}
}

pub fn screen_view(ctx: &Context, state: &mut ScreenViewState) {
	let image = ColorImage::from_rgba_unmultiplied([160, 144], &state.pixel_buffer);

	match &mut state.texture {
		None => {
			state.texture.insert(ctx.load_texture(
				"screen-texture",
				image,
				egui::TextureFilter::Nearest,
			));
		}
		Some(texture) => {
			texture.set(image, egui::TextureFilter::Nearest);
			egui::Window::new("Screen")
				.resizable(false)
				.show(ctx, |ui| {
					ui.add(Image::new(
						texture.id(),
						Vec2::new(160.0 * 2.0, 144.0 * 2.0),
					))
				});
		}
	}
}
