use std::mem;

use serde::{Deserialize, Serialize};

pub type Color = (u8, u8, u8, u8);

pub trait LCDDisplay {
	fn get_size(&self) -> (u8, u8);
	fn put_pixel(&mut self, x: u8, y: u8, color: Color);
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GameboyLCD {
	buffer_front: Vec<u8>,
	buffer_back: Vec<u8>,

	pub frame: u64,
	pub scale: f32,
}

impl LCDDisplay for GameboyLCD {
	fn get_size(&self) -> (u8, u8) {
		(160, 144)
	}

	fn put_pixel(&mut self, x: u8, y: u8, color: Color) {
		let (width, _) = self.get_size();

		let y = y as usize;
		let x = x as usize;
		let width = width as usize;

		let (r, g, b, a) = color;

		let index = (y * width + x) * 4;

		let image = self.get_back_buffer_mut();

		image[index] = r;
		image[index + 1] = g;
		image[index + 2] = b;
		image[index + 3] = a;
	}
}

impl GameboyLCD {
	pub fn swap_buffers(&mut self) {
		self.frame += 1;
		mem::swap(&mut self.buffer_front, &mut self.buffer_back);
	}

	fn get_back_buffer_mut(&mut self) -> &mut [u8] {
		return self.buffer_back.as_mut_slice();
	}

	pub fn front_buffer(&self) -> &[u8] {
		return self.buffer_front.as_slice();
	}
}

impl Default for GameboyLCD {
	fn default() -> Self {
		let buffer_front = vec![100; 160 * 144 * 4];
		let buffer_back = vec![100; 160 * 144 * 4];
		Self {
			buffer_front,
			buffer_back,
			frame: 0,
			scale: 3.0,
		}
	}
}
