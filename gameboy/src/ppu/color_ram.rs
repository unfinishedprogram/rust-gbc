use serde::{Deserialize, Serialize};

use crate::{lcd::Color, util::bits::BIT_7};

/// Handles reading and writing of color pallette data for CGB mode
#[derive(Clone, Serialize, Deserialize)]
pub struct ColorRamController {
	increment: bool,
	index: usize,
	data: [u16; 32],
	colors: [Color; 32],
}

impl Default for ColorRamController {
	fn default() -> Self {
		Self {
			data: [0; 32],
			colors: [(0, 0, 0, 255); 32],
			index: 0,
			increment: false,
		}
	}
}

impl ColorRamController {
	pub fn write_spec(&mut self, value: u8) {
		self.increment = value & BIT_7 == BIT_7;
		self.index = value as usize & 0b00111111;
	}

	pub fn read_spec(&self) -> u8 {
		let increment = if self.increment { BIT_7 } else { 0 };
		let index = self.index;
		increment | index as u8
	}

	pub fn read_data(&self) -> u8 {
		let val = self.data[self.index >> 1];

		match self.index & 1 {
			0 => ((val & 0xFF00) >> 8) as u8,
			1 => (val & 0x00FF) as u8,
			_ => unreachable!(),
		}
	}

	pub fn write_data(&mut self, value: u8) {
		let cur = &mut self.data[self.index / 2];

		match self.index & 1 {
			0 => *cur = (*cur & 0xFF00) | (value as u16),
			1 => *cur = (*cur & 0x00FF) | ((value as u16) << 8),
			_ => unreachable!(),
		}
		self.update_color(self.index / 2);

		// Only increment on writes, not reads
		if self.increment {
			self.index += 1;
			self.index &= 0b00111111;
		}
	}

	pub fn update_color(&mut self, index: usize) {
		let color = self.data[index];

		let r = color & 0b11111;
		let g = (color >> 5) & 0b11111;
		let b = (color >> 10) & 0b11111;

		let r = ((r << 3) | (r >> 2)) as u8;
		let g = ((g << 3) | (g >> 2)) as u8;
		let b = ((b << 3) | (b >> 2)) as u8;

		self.colors[index] = (r, g, b, 255);
	}

	pub fn get_color(&self, pallette: u8, color: u8) -> Color {
		self.colors[(pallette * 4 + color) as usize]
	}
}
