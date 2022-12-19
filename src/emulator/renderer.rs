type Color = (u8, u8, u8);

use crate::util::bits::*;
pub mod sprite;

use self::sprite::Sprite;

use super::{flags::LCDC, lcd::LCDDisplay, memory_mapper::MemoryMapper, ppu::PPU, EmulatorState};

pub trait Renderer {
	fn render_screen_pixel(
		&self,
		lcd: &mut dyn LCDDisplay,
		x: u8,
		y: u8,
		scanline_state: &ScanlineState,
		pixel_state: PixelState,
	);
	fn fetch_scanline_state(&mut self) -> ScanlineState;
	fn fetch_pixel_state(&self) -> PixelState;
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
	fn map_pallet_color(&self, pallet_addr: u16, color_index: u8) -> u8;
}

impl RendererHelpers for EmulatorState {
	fn fetch_sprites(&self) -> Vec<Sprite> {
		(0..40)
			.map(|i| {
				let index = 0xFE00 + i * 4;
				let bytes = (
					self.read(index),
					self.read(index + 1),
					self.read(index + 2),
					self.read(index + 3),
				);
				Sprite::new(index, bytes)
			})
			.filter(|sprite| sprite.is_visible())
			.collect()
	}

	fn get_sprite_pixel(&self, sprite: &Sprite, x: u8, y: u8) -> u8 {
		let x = if sprite.flip_x { x } else { 7 - x };

		if self.read(LCDC) & BIT_2 == 0 {
			// 8x8 Mode
			let y = if sprite.flip_y { y } else { 7 - y };
			self.get_tile_pixel_pallet_index(x, y, sprite.tile_index, true)
		} else {
			// 8x16 Mode
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

		let lcdc = self.read(LCDC);
		let indexing_mode = lcdc & BIT_4 != 0;

		let base = {
			let tile_bit = match tile_mode {
				TileMode::Window => BIT_6,
				TileMode::Background => BIT_3,
			};
			if lcdc & tile_bit != 0 {
				0x9C00
			} else {
				0x9800
			}
		};

		let tile_index = self.read(base + (x >> 3) + (y >> 3) * 32);

		let (tile_x, tile_y) = ((x % 8) as u8, (y % 8) as u8);
		self.get_tile_pixel_pallet_index(tile_x, tile_y, tile_index, indexing_mode)
	}

	fn get_bg_pixel_color(&self, x: u8, y: u8) -> Color {
		self.get_color_from_pallet_index(
			self.map_pallet_color(0xff47, self.get_pixel(TileMode::Background, x, y)),
		)
	}

	fn get_wn_pixel_color(&self, x: u8, y: u8) -> Color {
		self.get_color_from_pallet_index(
			self.map_pallet_color(0xff47, self.get_pixel(TileMode::Window, x, y)),
		)
	}

	fn get_color_from_pallet_index(&self, index: u8) -> Color {
		[(224, 248, 208), (136, 192, 112), (52, 104, 86), (8, 24, 32)][index as usize]
	}

	fn get_tile_pixel_pallet_index(&self, x: u8, y: u8, tile_index: u8, mode: bool) -> u8 {
		let (x, y) = (x as u16, y as u16);
		let (x, y) = (x.clamp(0, 7), y.clamp(0, 7));

		assert!(x <= 7 && y <= 7);

		let addr: u16 = if mode {
			0x8000 + 16 * tile_index as i32
		} else {
			0x9000 + 16 * (tile_index as i8) as i32
		} as u16;

		let bit_index = 7 - x as u8;

		let low = (self.read(addr + y * 2) & bit(bit_index)) >> bit_index;
		let high = (self.read(addr + y * 2 + 1) & bit(bit_index)) >> bit_index;
		low | (high << 1)
	}

	fn map_pallet_color(&self, pallet_addr: u16, color_index: u8) -> u8 {
		self.read(pallet_addr) >> (color_index * 2) & 0b11
	}
}

#[derive(Default, Clone)]
pub struct ScanlineState {
	pub lcdc: u8,
	pub wn_enabled: bool,
	pub w_index: u8,
	pub sprite_height: u8,
	pub sprites: Vec<Sprite>,
}

pub struct PixelState {
	pub bg_enabled: bool,
	pub sp_enabled: bool,
	pub lcdc: u8,
}

impl Renderer for EmulatorState {
	fn render_screen_pixel(
		&self,
		lcd: &mut dyn LCDDisplay,
		x: u8,
		y: u8,
		scanline_state: &ScanlineState,
		pixel_state: PixelState,
	) {
		let (scx, scy) = (self.read(0xFF43), self.read(0xFF42));
		let (wx, wy) = (self.read(0xFF4B), self.read(0xFF4A));

		let PixelState {
			lcdc: _,
			bg_enabled,
			sp_enabled,
		} = pixel_state;

		let ScanlineState {
			lcdc: _,
			wn_enabled,
			w_index,
			sprite_height: _,
			sprites,
		} = scanline_state;

		let wn_in_view = x + 7 >= wx && y >= wy;
		let wn_visible = wn_in_view && *wn_enabled;

		let mut base_color = if wn_visible {
			let x = x - wx + 7;
			let y = *w_index;
			self.get_pixel(TileMode::Window, x, y)
		} else if bg_enabled {
			let (x, y) = (x.wrapping_add(scx), y.wrapping_add(scy));
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

			let sprite_color = self.get_sprite_pixel(sprite, sprite.x - (x + 1), sprite.y - y - 9);

			if sprite_color == 0 {
				continue; // transparency
			}

			if sprite.above_bg || base_color == 0 {
				sprite_pixel = true;
				base_color = self.map_pallet_color(sprite.pallet_address, sprite_color);
			}
		}
		if sprite_pixel {
			lcd.put_pixel(x, y, self.get_color_from_pallet_index(base_color));
		} else {
			lcd.put_pixel(
				x,
				y,
				self.get_color_from_pallet_index(self.map_pallet_color(0xFF47, base_color)),
			);
		}
	}

	fn fetch_pixel_state(&self) -> PixelState {
		let lcdc = self.read(LCDC);
		let bg_enabled = lcdc & BIT_0 == BIT_0;
		let sp_enabled = lcdc & BIT_1 == BIT_1;

		PixelState {
			bg_enabled,
			sp_enabled,
			lcdc,
		}
	}

	fn fetch_scanline_state(&mut self) -> ScanlineState {
		let lcdc = self.read(LCDC);
		let line = self.get_ly();

		let bg_enabled = lcdc & BIT_0 == BIT_0;
		let wn_enabled = lcdc & BIT_5 == BIT_5 && bg_enabled;
		let sprite_height = 8 + if lcdc & BIT_2 == BIT_2 { 8 } else { 0 };

		let sprites = {
			let mut sprites: Vec<Sprite> = self
				.fetch_sprites()
				.into_iter()
				.filter(|sprite| sprite.y > line + 8 && sprite.y <= line + 8 + sprite_height)
				.take(10)
				.collect();

			sprites.sort();
			sprites
		};

		let (wx, wy) = (self.read(0xFF4B), self.read(0xFF4A));

		let w_index = self.ppu_state.window_line;
		if wn_enabled && line >= wy && wx < 144 - 7 {
			self.ppu_state.window_line += 1;
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
