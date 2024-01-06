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
		let get_tile_row = |tile_data, row| gb.ppu.get_tile_row(tile_data, row, 40);

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

				for row in 0..8 {
					let tile_data = TileData(index, Some(attributes));
					let tile_row = get_tile_row(tile_data, row);
					for pixel_index in 0..8 {
						let pixel = tile_row[pixel_index];
						let color = gb.ppu.bg_color(pixel);
						let color =
							Color32::from_rgba_premultiplied(color.0, color.1, color.2, color.3);
						let x = x * 8 + pixel_index as u16;
						let y = y * 8 + row as u16;
						img_out.image.pixels[y as usize * 256 + x as usize] = color;
					}
				}
			}
		}
	}

	fn render_tile_map(img_out: &mut TileImage, gb: &Gameboy, bank: u8) {
		let get_tile_row = |tile_data, row| gb.ppu.get_tile_row(tile_data, row, 40);

		for y in 0..24 {
			for x in 0..16 {
				let index = x + y * 16;
				let tile_index = index * 16;
				for row in 0..8 {
					let tile_data = TileData(tile_index, Some(TileAttributes::new(bank << 3)));
					let tile_row = get_tile_row(tile_data, row);
					for pixel_index in 0..8 {
						let pixel = tile_row[pixel_index];
						let color = gb.ppu.bg_color(pixel);
						let color =
							Color32::from_rgba_premultiplied(color.0, color.1, color.2, color.3);
						let x = x * 8 + pixel_index as u16;
						let y = y * 8 + row as u16;
						img_out.image.pixels[y as usize * 128 + x as usize] = color;
					}
				}
			}
		}
	}

	fn draw_tile_image(ui: &mut Ui, img: &mut TileImage) {
		let texture = img.texture.get_or_insert_with(|| {
			ui.ctx()
				.load_texture(img.name, img.image.clone(), TextureOptions::NEAREST)
		});

		texture.set(img.image.clone(), TextureOptions::NEAREST);

		ui.vertical(|ui| {
			ui.label(img.name);
			ui.add(Image::new(
				texture.id(),
				Vec2::new(img.image.width() as f32, img.image.height() as f32).mul(2.0),
			));
		});
	}

	fn render_images(&mut self, gameboy: &Gameboy) {
		VramView::render_tile_map(&mut self.tiles_bank_0, gameboy, 0);
		VramView::render_tile_map(&mut self.tiles_bank_1, gameboy, 1);
		VramView::render_win_bg_map(&mut self.bg_map, gameboy, FetcherMode::Background);
		VramView::render_win_bg_map(&mut self.win_map, gameboy, FetcherMode::Window);
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

			ui.separator();

			// Tile Data
			ui.horizontal(|ui| {
				VramView::draw_tile_image(ui, &mut self.tiles_bank_0);
				VramView::draw_tile_image(ui, &mut self.tiles_bank_1);
			});

			// BG/Window TileMaps
			ui.horizontal(|ui| {
				VramView::draw_tile_image(ui, &mut self.bg_map);
				VramView::draw_tile_image(ui, &mut self.win_map);
			});
		});
	}
}
