use std::ops::Mul;

use egui::{Color32, ColorImage, ComboBox, Image, TextureHandle, TextureOptions, Ui, Vec2};
use gameboy::{
	ppu::{
		renderer::{AddressingMode, PixelFIFO},
		tile_data::{TileAttributes, TileData},
		FetcherMode,
	},
	Gameboy,
};

pub struct TileImage {
	name: &'static str,
	texture: Option<TextureHandle>,
	image: ColorImage,
}

impl TileImage {
	fn new(size: [usize; 2], name: &'static str) -> Self {
		let color = egui::Color32::from_rgb(255, 255, 255);
		Self {
			name,
			texture: None,
			image: ColorImage::new(size, color),
		}
	}

	// x, y position is in tiles, not pixels
	fn draw_tile_data(&mut self, gb: &Gameboy, tile_data: TileData, x: u16, y: u16) {
		let img_width: usize = self.image.width();

		for row in 0..8 {
			let tile_row = gb.ppu.get_tile_row(tile_data, row, 40);
			for pixel_index in 0..8 {
				let pixel = tile_row[pixel_index];
				let color = gb.ppu.bg_color(pixel);
				let color = Color32::from_rgba_premultiplied(color.0, color.1, color.2, color.3);
				let x = x * 8 + pixel_index as u16;
				let y = y * 8 + row as u16;
				self.image.pixels[y as usize * img_width + x as usize] = color;
			}
		}
	}

	fn draw(&mut self, ui: &mut Ui) {
		let texture = self.texture.get_or_insert_with(|| {
			ui.ctx()
				.load_texture(self.name, self.image.clone(), TextureOptions::NEAREST)
		});

		texture.set(self.image.clone(), TextureOptions::NEAREST);

		ui.vertical(|ui| {
			ui.label(self.name);
			ui.add(Image::new(
				texture.id(),
				Vec2::new(self.image.width() as f32, self.image.height() as f32).mul(2.0),
			));
		});
	}
}

#[derive(Default, Debug, PartialEq)]
enum SelectedBank {
	#[default]
	Current,
	Bank0,
	Bank1,
}

pub struct VramView {
	tiles_bank_0: TileImage,
	tiles_bank_1: TileImage,

	bg_map: TileImage,
	win_map: TileImage,

	vram_bank: SelectedBank,
}

impl Default for VramView {
	fn default() -> Self {
		Self {
			tiles_bank_0: TileImage::new([128, 128 + 64], "tiles_bank_0"),
			tiles_bank_1: TileImage::new([128, 128 + 64], "tiles_bank_1"),
			bg_map: TileImage::new([256, 256], "bg_map"),
			win_map: TileImage::new([256, 256], "win_map"),
			vram_bank: SelectedBank::Current,
		}
	}
}

impl VramView {
	fn render_win_bg_map(img_out: &mut TileImage, gb: &Gameboy, mode: FetcherMode) {
		let addr_offset = gb.ppu.registers.lcdc.tile_map_offset(mode);

		for y in 0..32 {
			for x in 0..32 {
				let addr = addr_offset + x + y * 32;
				let index_byte = gb.ppu.v_ram_bank_0[addr as usize];
				let attr_byte = gb.ppu.v_ram_bank_1[addr as usize];
				let attributes = TileAttributes::new(attr_byte);

				let index = match gb.ppu.registers.lcdc.addressing_mode() {
					AddressingMode::Signed => 16 * index_byte as i32,
					AddressingMode::Unsigned => 0x1000 + 16 * (index_byte as i8) as i32,
				} as u16;

				let tile_data = TileData(index, Some(attributes));
				img_out.draw_tile_data(gb, tile_data, x, y);
			}
		}
	}

	fn render_tile_map(img_out: &mut TileImage, gb: &Gameboy, bank: u8) {
		for y in 0..24 {
			for x in 0..16 {
				let index = x + y * 16;
				let tile_index = index * 16;
				let tile_data = TileData(tile_index, Some(TileAttributes::new(bank << 3)));

				img_out.draw_tile_data(gb, tile_data, x, y);
			}
		}
	}

	fn render_images(&mut self, gameboy: &Gameboy) {
		VramView::render_tile_map(&mut self.tiles_bank_0, gameboy, 0);
		VramView::render_tile_map(&mut self.tiles_bank_1, gameboy, 1);
		VramView::render_win_bg_map(&mut self.bg_map, gameboy, FetcherMode::Background);
		VramView::render_win_bg_map(&mut self.win_map, gameboy, FetcherMode::Window);
	}

	fn draw_color_palettes(ui: &mut Ui, gameboy: &Gameboy, bg: bool) {
		const SCALE: f32 = 10.0;

		let color_ram = if bg {
			&gameboy.ppu.bg_color
		} else {
			&gameboy.ppu.obj_color
		};

		let draw_palette = |palette: u8, ui: &mut Ui| {
			let mut img_data: [u8; 16] = [255; 16];

			for color in 0..4 {
				let (r, g, b, a) = color_ram.get_color(palette, color);
				img_data[color as usize * 4 + 0] = r;
				img_data[color as usize * 4 + 1] = g;
				img_data[color as usize * 4 + 2] = b;
				img_data[color as usize * 4 + 3] = a;
			}

			let img = ColorImage::from_rgba_premultiplied([4, 1], &img_data);

			let texture = ui.ctx().load_texture(
				&format!("palette_{}", palette),
				img,
				TextureOptions::NEAREST,
			);

			ui.image(texture.id(), [4.0 * SCALE, 1.0 * SCALE]);
		};

		for palette in 0..8 {
			draw_palette(palette, ui);
		}
	}

	pub fn draw(&mut self, gameboy: &Gameboy, ui: &mut Ui) {
		self.render_images(gameboy);

		ui.vertical(|ui| {
			ComboBox::from_label("Bank")
				.selected_text(format!("{:?}", self.vram_bank))
				.show_ui(ui, |ui| {
					ui.selectable_value(&mut self.vram_bank, SelectedBank::Current, "Current");
					ui.selectable_value(&mut self.vram_bank, SelectedBank::Bank0, "Bank 0");
					ui.selectable_value(&mut self.vram_bank, SelectedBank::Bank1, "Bank 1");
				});

			ui.label("Background Palettes");
			VramView::draw_color_palettes(ui, gameboy, true);

			ui.label("OBJ Palettes");
			VramView::draw_color_palettes(ui, gameboy, false);
			ui.separator();

			// Tile Data
			ui.horizontal(|ui| {
				self.tiles_bank_0.draw(ui);
				self.tiles_bank_1.draw(ui);
			});

			// BG/Window TileMaps
			ui.horizontal(|ui| {
				self.bg_map.draw(ui);
				self.win_map.draw(ui);
			});
		});
	}
}
