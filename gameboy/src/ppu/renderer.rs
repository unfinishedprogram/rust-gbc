use serde::{Deserialize, Serialize};

use crate::util::bits::*;

use super::{
	sprite::Sprite,
	tile_data::{TileAttributes, TileData},
	FetcherMode, GBMode, VRAMBank, PPU,
};

#[derive(Clone, Serialize, Deserialize, Default, Copy)]
pub struct Pixel {
	/// a value between 0 and 3
	pub color: u8,
	/// on CGB a value between 0 and 7 and on DMG this only applies to sprites
	pub palette: u8,
	// on CGB this is the OAM index for the sprite and on DMG this doesn't exist
	sprite_priority: u16,
	// holds the value of the OBJ-to-BG Priority bit
	background_priority: bool,
}

impl Pixel {
	/// Creates a transparent pixel, with the lowest possible priority.
	///
	/// This is primarily for buffering the OBJ-FIFO
	#[inline]
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

pub enum SpriteHeight {
	Double,
	Single,
}

pub trait PixelFIFO {
	fn step_fifo(&mut self);
	fn push_pixel(&mut self);
	fn start_scanline(&mut self);
	fn get_tile_row(&self, tile_data: TileData, row: u8, sprite_priority: u16) -> [Pixel; 8];
	fn populate_bg_fifo(&mut self);
	fn in_window(&self) -> bool;
	fn get_tile_data(&self, tile_index: u16) -> TileData;
	fn start_window(&mut self);
	fn step_sprite_fifo(&mut self);
	fn push_sprite_pixels(&mut self, pixels: [Pixel; 8]);
	fn fetch_scanline_sprites(&mut self);
	fn draw_sprite(&mut self, sprite: Sprite);
}

impl PPU {
	// Returns the background color assigned to a given pixel
	// In CGB mode, this means accessing color ram
	// In DMG mode, this means accessing the background pallette
	pub fn bg_color(&self, pixel: Pixel) -> (u8, u8, u8, u8) {
		// Bottom two bits of BGP represent the color_id
		let color_id = (self.registers.bgp >> (pixel.color * 2)) & 0b11;

		match self.gb_mode {
			GBMode::CGB => self.bg_color.color_of(pixel),
			GBMode::DMG => self.dmg_pallette.color_of(color_id),
		}
	}

	// Returns the foreground color assigned to a given pixel
	// In CGB mode, this means accessing color ram
	// In DMG mode, this means accessing the background pallette
	pub fn obj_color(&self, pixel: Pixel) -> (u8, u8, u8, u8) {
		match self.gb_mode {
			GBMode::CGB => self.obj_color.color_of(pixel),
			GBMode::DMG => {
				let palette = match pixel.palette & 1 {
					0 => self.registers.obp0,
					1 => self.registers.obp1,
					_ => unreachable!(),
				};
				let color_id = (palette >> (pixel.color * 2)) & 0b11;

				self.dmg_pallette.color_of(color_id)
			}
		}
	}
}

impl PixelFIFO for PPU {
	/// Pushes OBJ Pixels onto the fifo.
	///
	/// Handles mixing and priorities of sprite pixels
	fn push_sprite_pixels(&mut self, pixels: [Pixel; 8]) {
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

	fn fetch_scanline_sprites(&mut self) {
		self.sprites.clear();
		let height = match self.registers.lcdc.obj_size() {
			SpriteHeight::Double => 0,
			SpriteHeight::Single => 8,
		};

		self.oam
			.chunks_exact(4)
			// Filter out sprites that are invisible due to position
			// This lets us not construct sprites that wont be drawn
			.filter(|chunk| {
				let x = chunk[1];
				let y = chunk[0];
				(x > 0 && y > 0 && x <= 168 && y <= 160)
					&& ((y > self.registers.ly.wrapping_add(height))
						&& (y <= self.registers.ly.wrapping_add(16)))
			})
			.enumerate()
			.map(|(index, bytes)| {
				Sprite::new(
					index as u16,
					bytes
						.try_into()
						.expect("Chunks should have exactly 4 elements each"),
				)
			})
			.take(10)
			.for_each(|it| self.sprites.push(it));

		// Sorting by the x-position means we only need to check a single sprite at a time
		self.sprites.sort_by(|a, b| a.x.cmp(&b.x).reverse());
	}

	/// Checks if the screen pixel currently being drawn is within the window
	///
	/// If this is true, the window should be drawn for the remainder of the current scanline
	fn in_window(&self) -> bool {
		let x_inside = self.current_pixel + 7 >= self.registers.wx;
		let y_inside = self.registers.ly >= self.registers.wy;
		let wn_enabled = self.registers.lcdc.win_enabled();

		y_inside && x_inside && wn_enabled
	}

	/// Start drawing the window
	/// Sets the fetcher mode to window for the remainder of the current scanline
	fn start_window(&mut self) {
		self.fifo_bg.clear();
		self.fetcher_mode = FetcherMode::Window;
		self.current_tile = 0;
		self.window_line = self.window_line.wrapping_add(1);
		// 6 dot penalty when transitioning fetching mode
		self.cycle += 6;
		self.populate_bg_fifo();
	}

	/// Initializes internal state to draw a new scanline
	///
	/// This is called at the start of each scanline and
	/// resets the fetcher mode to fetch background tiles
	fn start_scanline(&mut self) {
		self.fetcher_mode = FetcherMode::Background;
		self.current_pixel = 0;
		self.fifo_bg.clear();
		self.current_tile = self.registers.scx / 8;
		self.fetch_scanline_sprites();

		// Account for x-scroll of bg
		self.populate_bg_fifo();
		for _ in 0..self.registers.scx % 8 {
			self.fifo_bg.pop_front();
			self.cycle += 1;
		}

		for i in 0..8 {
			'_inner: loop {
				let Some(next) = self.sprites.last() else {
					break '_inner;
				};
				if next.x <= i {
					let Some(sprite) = self.sprites.pop() else {
						break '_inner;
					};
					self.draw_sprite(sprite)
				} else {
					break '_inner;
				}
			}
			self.fifo_obj.pop_back();
		}
	}

	fn draw_sprite(&mut self, sprite: Sprite) {
		let double_height = self.registers.lcdc.obj_size();
		if !self.registers.lcdc.obj_enable() {
			return;
		}

		let attributes = sprite.tile_attributes;

		let local_y = sprite.y.wrapping_sub(self.registers.ly).wrapping_sub(9);

		let tile_addr = match double_height {
			SpriteHeight::Double => {
				if !attributes.vertical_flip() ^ (local_y >= 8) {
					sprite.tile_index | 0x01
				} else {
					sprite.tile_index & 0xFE
				}
			}
			SpriteHeight::Single => sprite.tile_index,
		} as u16 * 16;

		let local_y = local_y & 7;

		let data = TileData(tile_addr, Some(attributes));

		if sprite.x == 0 {
			self.cycle += 11;
		} else {
			// TODO: Implement full OBJ penalty algorithm
			self.cycle += 6;
		}

		self.push_sprite_pixels(self.get_tile_row(data, local_y, sprite.addr));
	}

	fn step_sprite_fifo(&mut self) {
		let Some(next) = self.sprites.last() else {
			return;
		};

		if next.x == self.current_pixel.wrapping_add(8) {
			let Some(sprite) = self.sprites.pop() else {
				return;
			};
			self.draw_sprite(sprite);
			// Account for multiple sprites with the same X-position
			self.step_sprite_fifo();
		}
	}

	fn step_fifo(&mut self) {
		if matches!(self.fetcher_mode, FetcherMode::Background) && self.in_window() {
			self.start_window()
		}

		self.step_sprite_fifo();
		self.push_pixel();
	}

	/// Tries to push a pixel to the LCD
	fn push_pixel(&mut self) {
		let Some(bg) = self.fifo_bg.pop_front() else {
			return;
		};

		if self.fifo_bg.len() <= 8 {
			self.populate_bg_fifo();
		}

		let x = self.current_pixel;
		let y = self.registers.ly;

		self.current_pixel += 1;

		if let Some(fg) = self.fifo_obj.pop_back() {
			let bg_over = (!fg.background_priority || bg.background_priority) && bg.color != 0;
			let bg_over = bg_over && self.registers.lcdc.bg_enabled();
			let bg_over = bg_over || fg.color == 0;
			let pixel = if bg_over {
				self.bg_color(bg)
			} else {
				self.obj_color(fg)
			};
			self.lcd.put_pixel(x, y, pixel);
		} else {
			self.lcd.put_pixel(x, y, self.bg_color(bg));
		};
	}

	fn get_tile_row(&self, tile_data: TileData, row: u8, sprite_priority: u16) -> [Pixel; 8] {
		let row = row % 8;
		let TileData(index, attributes) = tile_data;

		let (row, horizontal_flip, background_priority, palette, bank) =
			if let Some(attributes) = attributes {
				let row = if attributes.vertical_flip() {
					7 - row
				} else {
					row
				};

				let horizontal_flip = attributes.horizontal_flip();

				let palette_number = match self.gb_mode {
					GBMode::DMG => attributes.dmg_palette(),
					GBMode::CGB => attributes.cgb_palette(),
				};

				(
					row,
					horizontal_flip,
					attributes.bg_priority(),
					palette_number,
					attributes.v_ram_bank(),
				)
			} else {
				(row, false, false, 0, VRAMBank::Bank0)
			};

		let bank = match bank {
			VRAMBank::Bank0 => &self.v_ram_bank_0,
			VRAMBank::Bank1 => &self.v_ram_bank_1,
		};

		let low = bank[index as usize + row as usize * 2];
		let high = bank[index as usize + row as usize * 2 + 1];

		let interleaved = interleave(low, high);

		let mut pixels: [Pixel; 8] = [Pixel {
			color: 0,
			palette,
			sprite_priority,
			background_priority,
		}; 8];

		let iter = pixels.iter_mut().enumerate();
		if horizontal_flip {
			for (i, pixel) in iter {
				pixel.color = ((interleaved >> (i * 2)) & 0b11) as u8
			}
		} else {
			for (i, pixel) in iter {
				pixel.color = ((interleaved >> ((7 - i) * 2)) & 0b11) as u8
			}
		}

		pixels
	}

	fn populate_bg_fifo(&mut self) {
		let tile_y = match self.fetcher_mode {
			FetcherMode::Background => (self.registers.ly.wrapping_add(self.registers.scy)) >> 3,
			FetcherMode::Window => self.window_line >> 3,
		};
		let tile_x = self.current_tile;
		self.current_tile = self.current_tile.wrapping_add(1) % 32;
		let tile_map_offset = self.registers.lcdc.tile_map_offset(self.fetcher_mode);

		let map_index = tile_x as u16 + tile_y as u16 * 32 + tile_map_offset;

		// SCY does not affect the window
		let scy_pixel_offset = match self.fetcher_mode {
			FetcherMode::Background => self.registers.scy,
			FetcherMode::Window => 0,
		};

		let tile_row = (self.registers.ly.wrapping_add(scy_pixel_offset)) % 8;

		let pixels = self.get_tile_row(self.get_tile_data(map_index), tile_row, 0);
		self.fifo_bg.extend(pixels.iter());
	}

	fn get_tile_data(&self, tile_index: u16) -> TileData {
		// Tile data address always comes from the first bank
		let data_addr = self.v_ram_bank_0[tile_index as usize];

		// Tile attributes come from second v-ram bank in CGB mode
		let attributes = match self.gb_mode {
			GBMode::DMG => None,
			GBMode::CGB => Some(TileAttributes::new(self.v_ram_bank_1[tile_index as usize])),
		};

		let index = match self.registers.lcdc.addressing_mode() {
			AddressingMode::Signed => 16 * data_addr as i32,
			AddressingMode::Unsigned => 0x1000 + 16 * (data_addr as i8) as i32,
		} as u16;

		TileData(index, attributes)
	}
}
