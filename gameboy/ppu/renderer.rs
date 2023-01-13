use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::{lcd::LCDDisplay, util::bits::*};

use super::{
	sprite::Sprite,
	tile_data::{self, TileAttributes, TileData},
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

pub enum AddressingMode {
	Signed,
	Unsigned,
}

pub trait PixelFIFO {
	/// Fetches a row of 8 background or window pixels and queues them up to be mixed with sprite pixels
	fn step_fifo(&mut self);
	fn push_pixel(&mut self);
	fn start_scanline(&mut self);
	fn get_tile_row(&self, tile_data: TileData, row: u8) -> Vec<Pixel>;
	fn populate_bg_fifo(&mut self);
	fn in_window(&self) -> bool;
	fn get_tile_map_offset(&self) -> u16;
	fn get_addressing_mode(&self) -> AddressingMode;
	fn get_tile_data(&self, tile_index: u16) -> TileData;
	fn start_window(&mut self);
	fn fetch_sprites(&mut self);
	fn step_sprite_fifo(&mut self);
}

impl PixelFIFO for PPU {
	fn fetch_sprites(&mut self) {
		self.sprites = (0..40)
			.map(|i| {
				let index = i * 4;
				let bytes = (
					self.oam[index],
					self.oam[index + 1],
					self.oam[index + 2],
					self.oam[index + 3],
				);

				Sprite::new(index as u16, bytes)
			})
			.filter(|sprite| sprite.is_visible())
			.collect()
	}

	fn in_window(&self) -> bool {
		let bg_enabled = self.lcdc & BIT_0 == BIT_0;
		let wn_enabled = self.lcdc & BIT_5 == BIT_5 && bg_enabled;
		let wn_in_view = self.current_pixel + 7 >= self.wx && self.ly >= self.wy;

		wn_in_view && wn_enabled
	}

	fn start_window(&mut self) {
		self.fifo_bg.clear();
		self.fetcher_mode = FetcherMode::Window;
		self.current_tile = 0;
		self.window_line = self.window_line.wrapping_add(1);
		self.populate_bg_fifo();
	}

	fn start_scanline(&mut self) {
		self.fetcher_mode = FetcherMode::Background;
		self.fifo_bg.clear();
		self.current_tile = self.scx >> 3;
		self.fetch_sprites();

		// Account for x-scroll of bg
		self.populate_bg_fifo();
		for _ in 0..self.scx % 8 {
			self.fifo_bg.pop_back();
		}
	}

	fn step_sprite_fifo(&mut self) {
		let double_height = self.lcdc & BIT_2 == BIT_2;

		for sprite in &self.sprites {
			if sprite.x != self.current_pixel.wrapping_add(7) {
				continue;
			}

			if double_height {
				if !((sprite.y > self.ly) && (sprite.y <= self.ly.wrapping_add(16))) {
					continue;
				}
			} else {
				if !((sprite.y > self.ly.wrapping_add(8)) && (sprite.y <= self.ly.wrapping_add(16)))
				{
					continue;
				}
			}

			self.fifo_obj.clear();

			let attributes = TileAttributes {
				vertical_flip: !sprite.flip_y,
				horizontal_flip: sprite.flip_x,
				v_ram_bank: sprite.tile_vram_bank as usize,
				bg_priority: sprite.above_bg,
				palette_number: sprite.pallette_number as usize,
			};

			// 0x1C00
			// 0x1800

			let local_y = sprite.y.wrapping_sub(self.ly).wrapping_sub(9);
			if double_height {
				if local_y >= 8 {
					let real_addr = ((sprite.tile_index | 0x01) as u16 * 16) as i32;

					let data = TileData(real_addr as u16, Some(attributes));

					for pix in self.get_tile_row(data, local_y - 8) {
						self.fifo_obj.push_front(pix);
					}
				} else {
					// Double height part
					let real_addr = ((sprite.tile_index & 0xFE) as u16 * 16) as i32;
					let data = TileData(real_addr as u16, Some(attributes));

					for pix in self.get_tile_row(data, local_y) {
						self.fifo_obj.push_front(pix);
					}
				}
			} else {
				let data = TileData(sprite.tile_index as u16 * 16, Some(attributes));

				for pix in self.get_tile_row(data, local_y) {
					self.fifo_obj.push_front(pix);
				}
			}
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

	fn push_pixel(&mut self) {
		if let Some(pixel) = self.fifo_bg.pop_back() {
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

	fn get_tile_row(&self, tile_data: TileData, row: u8) -> Vec<Pixel> {
		// debug_assert!(row < 8);
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
				sprite_priority: 0,
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
				self.current_tile = self.current_tile.wrapping_add(1);

				let map_index = tile_x as u16 + tile_y as u16 * 32 + self.get_tile_map_offset();

				let tile_row = (self.ly.wrapping_add(self.scy)) % 8;

				let pixels = self.get_tile_row(self.get_tile_data(map_index), tile_row);

				for pix in pixels {
					self.fifo_bg.push_front(pix);
				}
			}
			FetcherMode::Window => {
				let tile_y = self.window_line >> 3;
				let tile_x = self.current_tile;
				self.current_tile += 1;

				let map_index = tile_x as u16 + tile_y as u16 * 32 + self.get_tile_map_offset();

				let tile_row = self.window_line % 8;

				let pixels = self.get_tile_row(self.get_tile_data(map_index), tile_row);

				for pix in pixels {
					self.fifo_bg.push_front(pix);
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

// Get tile
// Get tile data low
// Get tile data high
// Sleep
// Push
