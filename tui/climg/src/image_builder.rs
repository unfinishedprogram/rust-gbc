use crate::{
	escape_codes::{
		cursor::{HIDE_CURSOR, RESET_COLORS, SHOW_CURSOR},
		EscapeCodeSupplier, EscapeCodes, ESC, NUM_CHAR,
	},
	img_to_colors::{to_colors, Color, ColorBlock},
};

pub struct ImageBuilder {
	current_fg: Color,
	current_bg: Color,
	width: usize,
	height: usize,
	cursor_x: usize,
	cursor_y: usize,
	current_img_buffer: Vec<ColorBlock>,
	escape_supplier: EscapeCodes,
	config: ImageBuilderConfig,
	buffer: Vec<u8>,
}

pub struct ImageBuilderConfig {
	pub skip_unchanged: bool,
}

impl ImageBuilder {
	pub fn new(width: usize, height: usize, config: ImageBuilderConfig) -> Self {
		let mut res = Self {
			cursor_x: 1,
			cursor_y: 1,
			current_fg: (0, 0, 0),
			current_bg: (0, 0, 0),
			width,
			height,
			buffer: vec![],
			current_img_buffer: vec![ColorBlock::default(); width * height / 2],
			escape_supplier: EscapeCodes,
			config,
		};

		res.begin();
		res
	}

	fn reset(&mut self) {
		self.write_raw(RESET_COLORS);
		self.write_raw(SHOW_CURSOR);
	}

	fn begin(&mut self) {
		self.buffer.clear();
		self.write_raw(HIDE_CURSOR);
		// Force a color reset
		{
			self.write_raw(&self.escape_supplier.fg((0, 0, 0)));
			self.write_raw(&self.escape_supplier.bg((0, 0, 0)));
			self.current_fg = (0, 0, 0);
			self.current_bg = (0, 0, 0);
		}

		self.move_to_force(1, 1);
	}

	pub fn write(&mut self, data: &str) {
		self.write_raw(data);
		self.step_write_head();
	}

	fn write_raw(&mut self, data: &str) {
		self.buffer.extend(data.as_bytes());
	}

	pub fn build(&mut self) -> String {
		self.reset();
		let string_value = String::from_utf8(self.buffer.clone()).unwrap();
		self.begin();
		string_value
	}

	pub fn move_to(&mut self, x: usize, y: usize) {
		if self.cursor_x == x && self.cursor_y == y {
			return;
		}

		self.move_to_force(x, y);
	}

	fn move_to_force(&mut self, x: usize, y: usize) {
		self.cursor_x = x;
		self.cursor_y = y;
		let (x, y) = (&NUM_CHAR[x], &NUM_CHAR[y]);
		self.write_raw(ESC);
		self.write_raw(y);
		self.write_raw(";");
		self.write_raw(x);
		self.write_raw("f");
	}

	fn set_color_fg(&mut self, color: Color) {
		let fg = self.escape_supplier.fg(color);
		if self.current_fg != color {
			self.write_raw(&fg);
		}
		self.current_fg = color;
	}

	fn set_color_bg(&mut self, color: Color) {
		let bg = self.escape_supplier.bg(color);
		if self.current_bg != color {
			self.write_raw(&bg);
		}
		self.current_bg = color;
	}

	fn set_color(&mut self, background: Color, foreground: Color) {
		self.set_color_bg(background);
		self.set_color_fg(foreground);
	}

	fn step_write_head(&mut self) {
		self.cursor_x += 1;
		if self.cursor_x > self.width {
			self.move_to(1, self.cursor_y + 1);
		}
	}

	fn draw_full_block(&mut self, color: Color) {
		if self.current_fg == color {
			self.write("█");
		} else if self.current_bg == color {
			self.write(" ");
		} else {
			self.set_color(self.current_bg, color);
			self.write("█");
		}
	}

	fn draw_color(&mut self, ColorBlock { top, bottom }: ColorBlock) {
		if top == bottom {
			self.draw_full_block(top);
		} else if bottom == self.current_fg && top == self.current_bg {
			self.write("▄");
		} else if top == self.current_fg && bottom == self.current_bg {
			self.write("▀");
		} else {
			self.set_color(top, bottom);
			self.write("▄");
		}
	}

	pub fn draw_img(&mut self, img_data: &[u8]) {
		let colors = to_colors(img_data, self.width, self.height);
		if self.config.skip_unchanged {
			self.draw_diff(&colors);
		} else {
			self.draw_full(&colors);
		}
	}

	fn draw_full(&mut self, colors: &[ColorBlock]) {
		for (index, &color) in colors.iter().enumerate() {
			self.current_img_buffer[index] = color;
			self.draw_color(color);
		}
	}

	fn draw_diff(&mut self, colors: &[ColorBlock]) {
		let mut skipped_last = false;
		for (index, &color) in colors.iter().enumerate() {
			if self.current_img_buffer[index] == color {
				skipped_last = true;
				self.step_write_head();
			} else {
				self.current_img_buffer[index] = color;
				if skipped_last {
					self.move_to_force(self.cursor_x, self.cursor_y);
					skipped_last = false;
				}
				self.draw_color(color);
			}
		}
	}
}
