use std::cell::RefCell;

use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::ppu;

use super::bitmap::bit_set;

type ScreenBuffer = [[[u8; 4]; 160]; 144];
type TileBuffer = [[[u8; 4]; 8]; 8];

pub fn put_tile(buffer: &mut ScreenBuffer, tile_data: TileBuffer, tile_x: usize, tile_y: usize) {
	let real_x = tile_x * 8;
	let real_y = tile_y * 8;

	for y in 0..8 {
		for x in 0..8 {
			buffer[real_y + y][real_x + x] = tile_data[y][x];
		}
	}
}

pub fn to_pixel_tile(gb_tile: [u8; 16]) -> TileBuffer {
	let mut buffer: TileBuffer = [[[255; 4]; 8]; 8];
	for y in 0..8 {
		for x in 0..8 {
			let color = match (
				bit_set(gb_tile[y * 2], x as u8),
				bit_set(gb_tile[y * 2 + 1], x as u8),
			) {
				(true, true) => [155, 188, 15, 255],
				(true, false) => [139, 172, 15, 255],
				(false, true) => [48, 98, 48, 255],
				(false, false) => [15, 56, 15, 255],
			};

			buffer[y][x] = color;
		}
	}
	return buffer;
}

pub fn debug_draw_tile_data(
	memory: &RefCell<Memory>,
	screen_buffer: &mut [[[u8; 4]; 160]; 144],
	page: usize,
) {
	let start = ppu::registers::PPURegister::VramStart as usize;
	let start = 18 * 20 * 8 * 2 * page + 1;

	let memory = memory.borrow();

	for y in 0..18 {
		for x in 0..20 {
			let index: u16 = (start + ((y * 20) + x) * 16) as u16;

			let mut values = [0; 16];

			for i in 0..16u16 {
				values[i as usize] = memory[index + i];
			}

			let tile_data = to_pixel_tile(values);
			put_tile(screen_buffer, tile_data, x, y);
		}
	}
}
