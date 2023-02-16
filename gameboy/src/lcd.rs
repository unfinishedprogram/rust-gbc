use serde::{Deserialize, Serialize};

pub type Color = (u8, u8, u8, u8);

pub trait LCDDisplay {
	fn get_size(&self) -> (u8, u8);
	fn put_pixel(&mut self, x: u8, y: u8, color: Color);
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LCD {
	buffers: Vec<Vec<u8>>,
	current_buffer: usize,
	pub frame: u64,
	pub scale: f32,
}

impl PartialEq for LCD {
	fn eq(&self, other: &Self) -> bool {
		self.frame == other.frame
	}
}

impl LCDDisplay for LCD {
	fn get_size(&self) -> (u8, u8) {
		(160, 144)
	}

	fn put_pixel(&mut self, x: u8, y: u8, color: Color) {
		let (width, height) = self.get_size();
		if x >= width || y >= height {
			return;
		}

		let (r, g, b, a) = color;

		let index: usize = y as usize * width as usize + x as usize;

		let image = &mut self.buffers[self.current_buffer];

		image[index * 4] = r;
		image[index * 4 + 1] = g;
		image[index * 4 + 2] = b;
		image[index * 4 + 3] = a;
	}
}

impl LCD {
	pub fn swap_buffers(&mut self) {
		self.frame += 1;
		self.current_buffer ^= 1;
	}

	pub fn get_current_as_bytes(&self) -> &[u8] {
		&self.buffers[self.current_buffer ^ 1]
	}
}

impl Default for LCD {
	fn default() -> Self {
		let buffers = vec![vec![100; 160 * 144 * 4], vec![255; 160 * 144 * 4]];
		Self {
			frame: 0,
			scale: 3.0,
			current_buffer: 0,
			buffers,
		}
	}
}
