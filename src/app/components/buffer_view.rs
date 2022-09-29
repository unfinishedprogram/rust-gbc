use egui::{ColorImage, Context, Image, TextureHandle, Vec2};

type PixelBuffer = Vec<Vec<[u8; 4]>>;

pub struct BufferViewState {
	pub pixel_buffer: PixelBuffer,
	size: (usize, usize),
	texture: Option<TextureHandle>,
	name: &'static str,
}

impl Default for BufferViewState {
	fn default() -> Self {
		BufferViewState::new("Screen", (160, 144))
	}
}

impl BufferViewState {
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

	return collector;
}

pub fn render_image(ctx: &Context, state: &mut BufferViewState) {
	let size = [state.size.0, state.size.1];

	let image = ColorImage::from_rgba_unmultiplied(size, to_flat(&state.pixel_buffer).as_slice());

	let texture = state.texture.get_or_insert_with(|| {
		ctx.load_texture(state.name, image.clone(), egui::TextureFilter::Nearest)
	});

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
