use crate::cpu;
use crate::ppu;

use super::bitmap::bit_set;

pub enum GBColors {}

pub fn put_tile(
	buffer: &mut [u8; 160 * 144 * 4],
	tile_data: [u8; 8 * 8 * 4],
	x_pos: usize,
	y_pos: usize,
) {
	// let base_index = (y_pos * 160 + x_pos) * 8 * 4;

	let real_x = x_pos * 8;
	let real_y = y_pos * 8;

	for y in 0..8 {
		for x in 0..8 {
			let real_index = ((real_x + x) + (real_y + y) * 160) * 4;

			let local_index = (y * 8 + x) * 4;

			buffer[real_index + 0] = tile_data[local_index + 0];
			buffer[real_index + 1] = tile_data[local_index + 1];
			buffer[real_index + 2] = tile_data[local_index + 2];
			buffer[real_index + 3] = tile_data[local_index + 3];
		}
	}
}

pub fn to_pixel_tile(gb_tile: [u8; 16]) -> [u8; 64 * 4] {
	let mut buffer = [255; 64 * 4];
	for y in 0..8 {
		for x in 0..8 {
			let color = match (
				bit_set(gb_tile[y * 2], x as u8),
				bit_set(gb_tile[y * 2 + 1], x as u8),
			) {
				(true, true) => (155, 188, 15),
				(true, false) => (139, 172, 15),
				(false, true) => (48, 98, 48),
				(false, false) => (15, 56, 15),
			};

			buffer[(y * 8 + x) * 4] = color.0;
			buffer[(y * 8 + x) * 4 + 1] = color.1;
			buffer[(y * 8 + x) * 4 + 2] = color.2;
		}
	}
	return buffer;
}

pub fn debug_draw_tile_data(cpu: &cpu::Cpu, screen_buffer: &mut [u8; 160 * 144 * 4], page: usize) {
	let start = ppu::registers::PPURegister::VramStart as usize;
	let start = 18 * 20 * 8 * 2 * page;

	for y in 0..18 {
		for x in 0..20 {
			let index = start + ((y * 20) + x) * 16;

			let mut values = [0; 16];

			for i in 0..16 {
				values[i] = cpu.memory[index + i];
			}

			let tile_data = to_pixel_tile(values);
			put_tile(screen_buffer, tile_data, x, y);
		}
	}
}
