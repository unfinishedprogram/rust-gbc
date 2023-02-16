use std::ops::Not;

use serde::{Deserialize, Serialize};

pub type Color = (u8, u8, u8, u8);

pub trait LCDDisplay {
	fn get_size(&self) -> (u8, u8);
	fn put_pixel(&mut self, x: u8, y: u8, color: Color);
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Buffer {
	A,
	B,
}

impl Not for Buffer {
	type Output = Buffer;
	fn not(self) -> Self::Output {
		match self {
			Buffer::A => Buffer::B,
			Buffer::B => Buffer::A,
		}
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GameboyLCD {
	buffers: (Vec<u8>, Vec<u8>),
	current_buffer: Buffer,
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

		let image = self.get_current_mut();

		image[index] = r;
		image[index + 1] = g;
		image[index + 2] = b;
		image[index + 3] = a;
	}
}

impl GameboyLCD {
	pub fn swap_buffers(&mut self) {
		self.frame += 1;
		self.current_buffer = !self.current_buffer;
	}

	pub fn get_current_as_bytes(&self) -> &[u8] {
		match self.current_buffer {
			Buffer::A => self.buffers.1.as_slice(),
			Buffer::B => self.buffers.0.as_slice(),
		}
	}

	fn get_current_mut(&mut self) -> &mut [u8] {
		match self.current_buffer {
			Buffer::A => self.buffers.0.as_mut_slice(),
			Buffer::B => self.buffers.1.as_mut_slice(),
		}
	}
}

impl Default for GameboyLCD {
	fn default() -> Self {
		let buffers = (vec![100; 160 * 144 * 4], vec![255; 160 * 144 * 4]);
		Self {
			frame: 0,
			scale: 3.0,
			current_buffer: Buffer::A,
			buffers,
		}
	}
}
