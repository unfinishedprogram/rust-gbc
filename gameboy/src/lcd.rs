use serde::{Deserialize, Serialize};

pub type Color = (u8, u8, u8, u8);

#[derive(Clone, Serialize, Deserialize)]
pub struct GameboyLCD {
	buffer_front: Vec<u8>,
	buffer_back: Vec<u8>,
	pub sync_mode: SyncMode,

	pub frame: u64,
	pub scale: f32,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum SyncMode {
	DoubleBuffered,
	None,
}

impl GameboyLCD {
	pub fn size(&self) -> (u8, u8) {
		(160, 144)
	}

	pub fn put_pixel(&mut self, x: u8, y: u8, color: Color) {
		let (width, _) = self.size();

		let y = y as usize;
		let x = x as usize;
		let width = width as usize;

		let (r, g, b, a) = color;

		let index = (y * width + x) * 4;

		let image = self.get_back_buffer_mut();

		image[index..index + 4].copy_from_slice(&[r, g, b, a])
	}

	pub fn swap_buffers(&mut self) {
		self.frame += 1;
		match self.sync_mode {
			SyncMode::DoubleBuffered => {
				std::mem::swap(&mut self.buffer_front, &mut self.buffer_back)
			}
			SyncMode::None => {}
		}
	}

	fn get_back_buffer_mut(&mut self) -> &mut [u8] {
		self.buffer_back.as_mut_slice()
	}

	pub fn front_buffer(&self) -> &[u8] {
		match self.sync_mode {
			SyncMode::DoubleBuffered => &self.buffer_front,
			SyncMode::None => &self.buffer_back,
		}
		.as_slice()
	}
}

impl Default for GameboyLCD {
	fn default() -> Self {
		let buffer_front = vec![100; 160 * 144 * 4];
		let buffer_back = vec![100; 160 * 144 * 4];
		Self {
			buffer_front,
			buffer_back,
			sync_mode: SyncMode::None,
			frame: 0,
			scale: 3.0,
		}
	}
}
