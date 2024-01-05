use egui::{Color32, ColorImage, ComboBox, Image, TextureHandle, TextureOptions, Ui, Vec2};
use gameboy::{
	ppu::{renderer::PixelFIFO, tile_data::TileData},
	Gameboy,
};

pub struct TileImage {
	texture: Option<TextureHandle>,
	image: ColorImage,
}

impl Default for TileImage {
	fn default() -> Self {
		let color = egui::Color32::from_rgb(255, 255, 255);

		Self {
			image: ColorImage::new([128, 128 + 64], color),
			texture: None,
		}
	}
}

#[derive(Default, Debug, PartialEq)]
enum SelectedBank {
	#[default]
	Current,
	Bank0,
	Bank1,
}

#[derive(Default)]
pub struct VramView {
	tiles: TileImage,
	vram_bank: SelectedBank,
}

impl VramView {
	fn render_image(&mut self, gameboy: &Gameboy) {
		self.tiles.image.pixels[0] = Color32::from_rgb(255, 255, 255);

		let get_tile_row = |tile_data, row| gameboy.ppu.get_tile_row(tile_data, row, 40);

		// for tile_index in 0..384 {
		// 	let tile_index = tile_index * 16;
		// }

		for y in 0..24 {
			for x in 0..16 {
				let index = x + y * 16;
				let tile_index = index * 16;
				for row in 0..8 {
					let tile_data = TileData(tile_index, None);
					let tile_row = get_tile_row(tile_data, row);
					for pixel_index in 0..8 {
						let pixel = tile_row[pixel_index];
						let color = gameboy.ppu.bg_color.color_of(pixel);
						let color =
							Color32::from_rgba_premultiplied(color.0, color.1, color.2, color.3);
						let x = x * 8 + pixel_index as u16;
						let y = y * 8 + row as u16;
						self.tiles.image.pixels[y as usize * 128 + x as usize] = color;
					}
				}
			}
		}
	}

	fn draw_info(ui: &mut Ui, _gameboy: &Gameboy) {
		ui.vertical(|ui| {
			ui.label("Info Goes Here");
		});
	}

	pub fn draw(&mut self, gameboy: &Gameboy, ui: &mut Ui) {
		self.render_image(gameboy);

		let texture = self.tiles.texture.get_or_insert_with(|| {
			ui.ctx()
				.load_texture("screen", self.tiles.image.clone(), TextureOptions::NEAREST)
		});

		texture.set(self.tiles.image.clone(), TextureOptions::NEAREST);

		ui.vertical(|ui| {
			ComboBox::from_label("Bank")
				.selected_text(format!("{:?}", self.vram_bank))
				.show_ui(ui, |ui| {
					ui.selectable_value(&mut self.vram_bank, SelectedBank::Current, "Current");
					ui.selectable_value(&mut self.vram_bank, SelectedBank::Bank0, "Bank 0");
					ui.selectable_value(&mut self.vram_bank, SelectedBank::Bank1, "Bank 1");
				});

			ui.separator();
			ui.horizontal(|ui| {
				VramView::draw_info(ui, gameboy);
				ui.add(Image::new(
					texture.id(),
					Vec2::new(128.0 * 2.0, (128.0 + 64.0) * 2.0),
				));
			});
		});
	}
}
