use serde::{Deserialize, Serialize};

use crate::{lcd::LCDDisplay, util::bits::*};

use super::{
	sprite::Sprite,
	tile_data::{TileAttributes, TileData},
	FetcherMode, PPU,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Pixel {
	/// a value between 0 and 3
	color: u8,
	/// on CGB a value between 0 and 7 and on DMG this only applies to sprites
	palette: u8,
	// on CGB this is the OAM index for the sprite and on DMG this doesn't exist
	sprite_priority: usize,
	// holds the value of the OBJ-to-BG Priority bit
	background_priority: bool,
}

impl Pixel {
	/// Creates a transparent pixel, with the lowest possible priority.
	///
	/// This is primarily for buffering the OBJ-FIFO
	pub fn transparent() -> Self {
		Self {
			color: 0,
			palette: 0,
			sprite_priority: 40,
			background_priority: false,
		}
	}
}

pub enum AddressingMode {
	Signed,
	Unsigned,
}

pub trait PixelFIFO {
	fn step_fifo(&mut self);
	fn push_pixel(&mut self);
	fn start_scanline(&mut self);
	fn get_tile_row(&self, tile_data: TileData, row: u8, sprite_priority: usize) -> Vec<Pixel>;
	fn populate_bg_fifo(&mut self);
	fn in_window(&self) -> bool;
	fn get_tile_map_offset(&self) -> u16;
	fn get_addressing_mode(&self) -> AddressingMode;
	fn get_tile_data(&self, tile_index: u16) -> TileData;
	fn start_window(&mut self);
	fn step_sprite_fifo(&mut self);
	fn push_sprite_pixels(&mut self, pixels: Vec<Pixel>);
	fn fetch_scanline_sprites(&self) -> Vec<Sprite>;
}

impl PixelFIFO for PPU {
	/// Pushes OBJ Pixels onto the fifo.
	///
	/// Handles mixing and priorities of sprite pixels
	fn push_sprite_pixels(&mut self, pixels: Vec<Pixel>) {
		// Fill fifo with transparent pixels until it has 8

		while self.fifo_obj.len() < 8 {
			self.fifo_obj.push_front(Pixel::transparent());
		}

		// Mix the new pixels with those currently in the FIFO
		for (i, pixel) in pixels.into_iter().enumerate() {
			let other = &mut self.fifo_obj[i];
			if (pixel.color != 0 && pixel.sprite_priority < other.sprite_priority)
				|| other.color == 0
			{
				*other = pixel;
			}
		}
	}

	fn fetch_scanline_sprites(&self) -> Vec<Sprite> {
		let double_height = if self.lcdc & BIT_2 == BIT_2 { 0 } else { 8 };

		self.oam
			.chunks_exact(4)
			.enumerate()
			.map(|(index, bytes)| {
				Sprite::new(
					index as u16,
					bytes
						.try_into()
						.expect("Chunks should have exactly 4 elements each"),
				)
			})
			.filter(|sprite| sprite.is_visible())
			.filter(|sprite| {
				(sprite.y > self.ly.wrapping_add(double_height))
					&& (sprite.y <= self.ly.wrapping_add(16))
			})
			.take(10)
			.collect()
	}

	/// Checks if the screen pixel currently being drawn is within the window
	///
	/// If this is true, the window should be drawn for the remainder of the current scanline
	fn in_window(&self) -> bool {
		let bg_enabled = self.lcdc & BIT_0 == BIT_0;
		let wn_enabled = self.lcdc & BIT_5 == BIT_5 && bg_enabled;
		let wn_in_view = self.current_pixel + 7 >= self.wx && self.ly >= self.wy;

		wn_in_view && wn_enabled
	}

	/// Start drawing the window
	/// Sets the fetcher mode to window for the remainder of the current scanline
	fn start_window(&mut self) {
		self.fifo_bg.clear();
		self.fetcher_mode = FetcherMode::Window;
		self.current_tile = 0;
		self.window_line = self.window_line.wrapping_add(1);
		self.populate_bg_fifo();
	}

	/// Initializes internal state to draw a new scanline
	///
	/// This is called at the start of each scanline and
	/// resets the fetcher mode to fetch background tiles
	fn start_scanline(&mut self) {
		self.fetcher_mode = FetcherMode::Background;
		self.fifo_bg.clear();
		self.current_tile = self.scx / 8;
		self.sprites = self.fetch_scanline_sprites();

		// Account for x-scroll of bg
		self.populate_bg_fifo();
		for _ in 0..self.scx % 8 {
			self.fifo_bg.pop_front();
		}

		// TODO, handle sprites within the first line, I.E. X <= 7
	}

	fn step_sprite_fifo(&mut self) {
		let double_height = self.lcdc & BIT_2 == BIT_2;

		let mut tiles: Vec<Vec<Pixel>> = vec![];

		for sprite in &self.sprites {
			if sprite.x != self.current_pixel.wrapping_add(7) {
				continue;
			}
			let attributes = TileAttributes {
				vertical_flip: !sprite.flip_y,
				horizontal_flip: !sprite.flip_x,
				v_ram_bank: sprite.tile_vram_bank as usize,
				bg_priority: sprite.above_bg,
				palette_number: sprite.pallette_number as usize,
			};

			let local_y = sprite.y.wrapping_sub(self.ly).wrapping_sub(9);
			if double_height {
				if local_y >= 8 {
					let real_addr = ((sprite.tile_index | 0x01) as u16 * 16) as i32;

					let data = TileData(real_addr as u16, Some(attributes));
					tiles.push(self.get_tile_row(data, local_y - 8, sprite.addr as usize));
				} else {
					// Double height part
					let real_addr = ((sprite.tile_index & 0xFE) as u16 * 16) as i32;
					let data = TileData(real_addr as u16, Some(attributes));
					tiles.push(self.get_tile_row(data, local_y, sprite.addr as usize));
				}
			} else {
				let data = TileData(sprite.tile_index as u16 * 16, Some(attributes));
				tiles.push(self.get_tile_row(data, local_y, sprite.addr as usize));
			}
		}

		for pixels in tiles {
			self.push_sprite_pixels(pixels);
		}
	}

	fn step_fifo(&mut self) {
		if matches!(self.fetcher_mode, FetcherMode::Background) && self.in_window() {
			self.start_window()
		}

		self.step_sprite_fifo();

		self.push_pixel();

		if self.fifo_bg.len() <= 8 {
			self.populate_bg_fifo();
		}
	}

	/// Tries to push a pixel to the LCD
	fn push_pixel(&mut self) {
		if let Some(pixel) = self.fifo_bg.pop_front() {
			let x = self.current_pixel;
			let y = self.ly;

			if let Some(lcd) = &mut self.lcd {
				lcd.put_pixel(x, y, self.bg_color.get_color(pixel.palette, pixel.color));

				if let Some(pixel) = self.fifo_obj.pop_back() {
					if !(pixel.color == 0) {
						lcd.put_pixel(x, y, self.obj_color.get_color(pixel.palette, pixel.color));
					}
				}
			}
			self.current_pixel += 1;
		}
	}

	fn get_tile_map_offset(&self) -> u16 {
		if self.lcdc
			& match self.fetcher_mode {
				FetcherMode::Window => BIT_6,
				FetcherMode::Background => BIT_3,
			} != 0
		{
			0x1C00
		} else {
			0x1800
		}
	}

	fn get_tile_row(&self, tile_data: TileData, row: u8, sprite_priority: usize) -> Vec<Pixel> {
		let row = row % 8;
		let TileData(index, attributes) = tile_data;

		let (row, horizontal_flip, background_priority, palette, bank) =
			if let Some(attributes) = &attributes {
				let row = if attributes.vertical_flip {
					7 - row
				} else {
					row
				};

				let horizontal_flip = attributes.horizontal_flip;

				(
					row,
					horizontal_flip,
					attributes.bg_priority,
					attributes.palette_number as u8,
					attributes.v_ram_bank,
				)
			} else {
				(row, false, false, 0, 0)
			};

		let low = self.v_ram[bank][index as usize + row as usize * 2];
		let high = self.v_ram[bank][index as usize + row as usize * 2 + 1];
		let interleaved = interleave(low, high);

		let pixels = (0..8).map(|index| {
			let color = (interleaved >> (index * 2) & 0b11) as u8;
			Pixel {
				color,
				palette,
				sprite_priority,
				background_priority,
			}
		});

		if horizontal_flip {
			pixels.collect()
		} else {
			pixels.rev().collect()
		}
	}

	fn populate_bg_fifo(&mut self) {
		match self.fetcher_mode {
			FetcherMode::Background => {
				let tile_y = (self.ly.wrapping_add(self.scy)) >> 3;

				let tile_x = self.current_tile;
				self.current_tile = self.current_tile.wrapping_add(1) % 32;

				let map_index = tile_x as u16 + tile_y as u16 * 32 + self.get_tile_map_offset();

				let tile_row = (self.ly.wrapping_add(self.scy)) % 8;

				let pixels = self.get_tile_row(self.get_tile_data(map_index), tile_row, 0);

				for pix in pixels {
					self.fifo_bg.push_back(pix);
				}
			}
			FetcherMode::Window => {
				let tile_y = self.window_line >> 3;
				let tile_x = self.current_tile;
				self.current_tile += 1;

				let map_index = tile_x as u16 + tile_y as u16 * 32 + self.get_tile_map_offset();

				let tile_row = self.window_line % 8;

				let pixels = self.get_tile_row(self.get_tile_data(map_index), tile_row, 0);

				for pix in pixels {
					self.fifo_bg.push_back(pix);
				}
			}
		}
	}

	fn get_addressing_mode(&self) -> AddressingMode {
		if self.lcdc & BIT_4 != 0 {
			AddressingMode::Signed
		} else {
			AddressingMode::Unsigned
		}
	}

	fn get_tile_data(&self, tile_index: u16) -> TileData {
		// Tile data address always comes from the first bank
		let data_addr = self.v_ram[0][tile_index as usize];

		// Tile attributes come from second v-ram bank in CGB mode
		let attributes = self.v_ram[1][tile_index as usize];

		let index = match self.get_addressing_mode() {
			AddressingMode::Signed => 16 * data_addr as i32,
			AddressingMode::Unsigned => 0x1000 + 16 * (data_addr as i8) as i32,
		} as u16;

		TileData(index, Some(TileAttributes::new(attributes)))
	}
}
