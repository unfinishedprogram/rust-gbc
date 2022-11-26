type Color = (u8, u8, u8);
use crate::util::{bit_ops::get_bit, bits::*};
pub mod sprite;

use self::sprite::Sprite;

use super::{lcd::LCDDisplay, memory_mapper::MemoryMapper, EmulatorState};

pub trait Renderer {
	fn render(&mut self, lcd: &mut dyn LCDDisplay);
}

trait RendererHelpers {
	fn get_bg_pixel(&mut self, x: u8, y: u8) -> Color;
	fn get_wn_pixel(&mut self, x: u8, y: u8) -> Color;
	fn get_tile_pixel(&mut self, x: u8, y: u8, tile_index: u8, mode: bool) -> Color;
	fn render_sprites(&mut self, lcd: &mut dyn LCDDisplay);
	fn fetch_sprites(&mut self) -> Vec<Sprite>;
	fn get_sprite_pixel(&mut self, sprite: &Sprite, x: u8, y: u8) -> Color;
}

impl RendererHelpers for EmulatorState {
	fn fetch_sprites(&mut self) -> Vec<Sprite> {
		vec![Sprite::new((
			self.read(0xFE00),
			self.read(0xFE01),
			self.read(0xFE02),
			self.read(0xFE03),
		))]

		// (0..40)
		// 	.map(|i| {
		// 		let index = 0xFE00 + i * 4;
		// 		Sprite::new((
		// 			self.read(index + 0),
		// 			self.read(index + 1),
		// 			self.read(index + 2),
		// 			self.read(index + 3),
		// 		))
		// 	})
		// 	.collect()
	}

	fn render_sprites(&mut self, lcd: &mut dyn LCDDisplay) {
		// Get only the visible sprites
		let sprites = self.fetch_sprites().into_iter().filter(|s| s.is_visible());

		for sprite in sprites {
			log::error!("Rendering Sprite");
			for x in sprite.x..sprite.x + 8 {
				for y in sprite.y..sprite.y + 8 {
					log::error!("Drawing Sprite");
					if y <= 16 || x <= 8 {
						continue;
					}

					lcd.put_pixel(x - 8, y - 16, self.get_sprite_pixel(&sprite, x, y));
				}
			}
		}
	}

	fn get_sprite_pixel(&mut self, sprite: &Sprite, x: u8, y: u8) -> Color {
		let x = if sprite.flip_x { 8 - x } else { x };
		let y = if sprite.flip_y { 8 - y } else { y };

		self.get_tile_pixel(x, y, sprite.tile_index, true)
	}

	fn get_bg_pixel(&mut self, x: u8, y: u8) -> Color {
		let lcdc = self.read(0xFF40);

		let (x, y) = (x as u16, y as u16);

		let indexing_mode = lcdc & BIT_4 != 0;

		let tile_mode = lcdc & BIT_3 != 0;
		let base: u16 = if tile_mode { 0x9C00 } else { 0x9800 };

		let tile_index = self.read(base + (x >> 3) + (y >> 3) * 32);

		let (tile_x, tile_y) = ((x % 8) as u8, (y % 8) as u8);

		self.get_tile_pixel(tile_x, tile_y, tile_index, indexing_mode)
	}

	fn get_wn_pixel(&mut self, x: u8, y: u8) -> Color {
		let lcdc = self.read(0xFF40);
		let (x, y) = (x as u16, y as u16);

		let indexing_mode = lcdc & BIT_4 != 0;

		let tile_mode = lcdc & BIT_6 != 0;
		let base: u16 = if tile_mode { 0x9C00 } else { 0x9800 };

		let tile_index = self.read(base + (x >> 3) + (y >> 3) * 32);

		let (tile_x, tile_y) = ((x % 8) as u8, (y % 8) as u8);

		self.get_tile_pixel(tile_x, tile_y, tile_index, indexing_mode)
	}

	fn get_tile_pixel(&mut self, x: u8, y: u8, tile_index: u8, mode: bool) -> Color {
		let (x, y) = (x as u16, y as u16);

		let addr: u16 = if mode {
			0x8000 + 16 * tile_index as i32
		} else {
			0x9000 + 16 * (tile_index as i8) as i32
		} as u16;

		match (
			get_bit(self.read(addr + y * 2), x as u8),
			get_bit(self.read(addr + y * 2 + 1), x as u8),
		) {
			(true, true) => (8, 24, 32),
			(true, false) => (224, 248, 208),
			(false, true) => (52, 104, 86),
			(false, false) => (136, 192, 112),
		}
	}
}

impl Renderer for EmulatorState {
	fn render(&mut self, lcd: &mut dyn LCDDisplay) {
		// Bit 7 - LCD Display Enable             (0=Off, 1=On)
		// Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
		// Bit 5 - Window Display Enable          (0=Off, 1=On)
		// Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
		// Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
		// Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
		// Bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)
		// Bit 0 - BG/Window Display/Priority     (0=Off, 1=On)

		// let lcdc = self.read(0xFF40);

		// let lcd_enable = lcdc & 0b00000001 == 0b00000001;
		// let window_tile_map_display_sel = lcdc & 0b00000010 == 0b00000010;
		// let window_display_enable = lcdc & 0b00000100 == 0b00000100;
		// let bg_win_tile_map_sel = lcdc & 0b00001000 == 0b00001000;
		// let bg_tile_map_sel = lcdc & 0b00010000 == 0b00010000;
		// let sprite_size = lcdc & 0b00100000 == 0b00100000;
		// let sprite_enable = lcdc & 0b01000000 == 0b01000000;
		// let bg_window_priority = lcdc & 0b10000000 == 0b10000000;

		let (scx, scy) = (self.read(0xFF43), self.read(0xFF42));
		let (wx, wy) = (self.read(0xFF4B), self.read(0xFF4A));

		for y in 0u8..144 {
			for x in 0u8..160 {
				let bg = self.get_bg_pixel(x.wrapping_add(scx), y.wrapping_add(scy));
				lcd.put_pixel(x, y, bg);
				// if window_display_enable {
				// let wn = self.get_wn_pixel(x - wx, y - wy);
				// match wn {
				// (224, 248, 208) => {}
				// (_, _, _) => lcd.put_pixel(x, y, wn),
				// }
				// }
			}
		}

		self.render_sprites(lcd);
	}
}
