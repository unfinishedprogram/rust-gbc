pub type Color = (u8, u8, u8, u8);

use serde::{Deserialize, Serialize};

use super::sprite::Sprite;
use crate::{lcd::LCDDisplay, ppu::PPU, util::bits::*};

pub trait Renderer {
	fn render_screen_pixel(&mut self, x: u8, y: u8);
	fn fetch_scanline_state(&mut self) -> ScanlineState;
}

enum TileMode {
	Window,
	Background,
}

trait RendererHelpers {
	fn get_pixel(&self, tile_mode: TileMode, x: u8, y: u8) -> u8;

	fn get_bg_pixel_color(&self, x: u8, y: u8) -> Color;
	fn get_wn_pixel_color(&self, x: u8, y: u8) -> Color;

	fn get_tile_pixel_pallet_index(&self, x: u8, y: u8, tile_index: u8, mode: bool) -> u8;
	fn fetch_sprites(&self) -> Vec<Sprite>;
	fn get_sprite_pixel(&self, sprite: &Sprite, x: u8, y: u8) -> u8;
	fn get_color_from_pallet_index(&self, index: u8) -> Color;
	fn map_pallet_color(&self, pallet_addr: bool, color_index: u8) -> u8;
}

impl RendererHelpers for PPU {
	fn fetch_sprites(&self) -> Vec<Sprite> {
		(0..40)
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

	fn get_sprite_pixel(&self, sprite: &Sprite, x: u8, y: u8) -> u8 {
		let x = if sprite.flip_x { x } else { 7 - x };
		if self.lcdc & BIT_2 == 0 {
			// 8x8 Mode
			let y = if sprite.flip_y { y } else { 7 - y };
			self.get_tile_pixel_pallet_index(x, y, sprite.tile_index, true)
		} else {
			// 8x16 Mode
			let y = y.wrapping_add(16);
			let y = if sprite.flip_y { y } else { 15 - y };
			if y < 8 {
				self.get_tile_pixel_pallet_index(x, y, sprite.tile_index & 0xFE, true)
			} else {
				self.get_tile_pixel_pallet_index(x, y - 8, sprite.tile_index | 0x01, true)
			}
		}
	}

	fn get_pixel(&self, tile_mode: TileMode, x: u8, y: u8) -> u8 {
		let (x, y) = (x as u16, y as u16);

		let lcdc = &self.lcdc;
		let indexing_mode = lcdc & BIT_4 != 0;

		let base = {
			let tile_bit = match tile_mode {
				TileMode::Window => BIT_6,
				TileMode::Background => BIT_3,
			};
			if lcdc & tile_bit != 0 {
				0x9C00 - 0x8000
			} else {
				0x9800 - 0x8000
			}
		};

		let tile_index = self.v_ram[0][(base + (x >> 3) + (y >> 3) * 32) as usize];

		let (tile_x, tile_y) = ((x % 8) as u8, (y % 8) as u8);
		self.get_tile_pixel_pallet_index(tile_x, tile_y, tile_index, indexing_mode)
	}

	fn get_bg_pixel_color(&self, x: u8, y: u8) -> Color {
		self.get_color_from_pallet_index(
			self.bgp >> (self.get_pixel(TileMode::Background, x, y) * 2) & 0b11,
		)
	}

	fn get_wn_pixel_color(&self, x: u8, y: u8) -> Color {
		self.get_color_from_pallet_index(
			self.bgp >> (self.get_pixel(TileMode::Window, x, y) * 2) & 0b11,
		)
	}

	fn get_color_from_pallet_index(&self, index: u8) -> Color {
		[
			(0xFF, 0xFF, 0xFF, 0xFF),
			(0xAA, 0xAA, 0xAA, 0xFF),
			(0x55, 0x55, 0x55, 0xFF),
			(0x00, 0x00, 0x00, 0xFF),
		][index as usize]
	}

	fn get_tile_pixel_pallet_index(&self, x: u8, y: u8, tile_index: u8, mode: bool) -> u8 {
		let (x, y) = (x as u16, y as u16);
		let (x, y) = (x.clamp(0, 7), y.clamp(0, 7));

		assert!(x <= 7 && y <= 7);

		let addr: u16 = if mode {
			16 * tile_index as i32
		} else {
			0x1000 + 16 * (tile_index as i8) as i32
		} as u16;

		let bit_index = 7 - x as u8;

		let low = (self.v_ram[0][(addr + y * 2) as usize] & bit(bit_index)) >> bit_index;
		let high = (self.v_ram[0][(addr + y * 2 + 1) as usize] & bit(bit_index)) >> bit_index;
		low | (high << 1)
	}

	fn map_pallet_color(&self, pallet_addr: bool, color_index: u8) -> u8 {
		let palette_val = if pallet_addr { self.obp1 } else { self.obp0 };
		palette_val >> (color_index * 2) & 0b11
	}
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct ScanlineState {
	pub lcdc: u8,
	pub wn_enabled: bool,
	pub w_index: u8,
	pub sprite_height: u8,
	pub sprites: Vec<Sprite>,
}

impl Renderer for PPU {
	fn render_screen_pixel(&mut self, x: u8, y: u8) {
		let ScanlineState {
			lcdc,
			wn_enabled,
			w_index,
			sprite_height,
			sprites,
		} = &self.scanline_state;

		let bg_enabled = lcdc & BIT_0 == BIT_0;
		let sp_enabled = lcdc & BIT_1 == BIT_1;

		let wn_in_view = x + 7 >= self.wx && y >= self.wy;
		let wn_visible = wn_in_view && *wn_enabled;

		let mut base_color = if wn_visible {
			let x = x.wrapping_sub(self.wx).wrapping_add(7);
			let y = *w_index;
			self.get_pixel(TileMode::Window, x, y)
		} else if bg_enabled {
			let (x, y) = (x.wrapping_add(self.scx), y.wrapping_add(self.scy));
			self.get_pixel(TileMode::Background, x, y)
		} else {
			0
		};

		let mut sprite_pixel = false;
		for sprite in sprites {
			if !sp_enabled {
				break;
			}

			if x >= sprite.x || sprite.x >= x + 9 {
				continue; // Not inside sprite
			}

			let sprite_color = {
				let x = sprite.x.wrapping_sub(x + 1);
				let y = sprite
					.y
					.wrapping_sub(y)
					.wrapping_sub(*sprite_height)
					.wrapping_sub(1);

				self.get_sprite_pixel(sprite, x, y)
			};

			if sprite_color == 0 {
				continue; // transparency
			}

			if sprite.above_bg || base_color == 0 {
				sprite_pixel = true;
				base_color = self.map_pallet_color(sprite.pallet_address, sprite_color);
			}
		}

		let color = if sprite_pixel {
			self.get_color_from_pallet_index(base_color)
		} else {
			self.get_color_from_pallet_index(self.bgp >> (base_color * 2) & 0b11)
		};

		let Some(lcd) = self.lcd.as_mut() else {
			return;
		};

		lcd.put_pixel(x, y, color);
	}

	fn fetch_scanline_state(&mut self) -> ScanlineState {
		let lcdc = self.lcdc;
		let line = self.ly;

		let bg_enabled = lcdc & BIT_0 == BIT_0;
		let wn_enabled = lcdc & BIT_5 == BIT_5 && bg_enabled;
		let sprite_height = if lcdc & BIT_2 == BIT_2 { 16 } else { 8 };
		let dhs = lcdc & BIT_2 == BIT_2;

		let sprites = {
			let mut sprites: Vec<Sprite> = self
				.fetch_sprites()
				.into_iter()
				.filter(|sprite| {
					if dhs {
						(sprite.y > line) && (sprite.y <= line + 16)
					} else {
						(sprite.y > line + 8) && (sprite.y <= line + 16)
					}
				})
				.take(10)
				.collect();

			sprites.sort();
			sprites
		};

		let w_index = self.window_line;

		if wn_enabled && line >= self.wy && self.wx < 144 - 7 {
			self.window_line += 1;
		}

		ScanlineState {
			lcdc,
			wn_enabled,
			w_index,
			sprite_height,
			sprites,
		}
	}
}
