use crate::{
	emulator::{
		flags,
		flags::{get_bit_flag, set_bit_flag, set_bit_flag_to, BitFlag, STATFlag},
		io_registers::IORegistersAdress,
		memory_mapper::MemoryMapper,
	},
	util::bit_ops::get_bit,
};

use super::{lcd::LCDDisplay, EmulatorState};

type Color = (u8, u8, u8);

#[derive(Debug)]
pub enum PPUMode {
	HBlank = 0,
	VBlank = 1,
	OamScan = 2,
	Draw = 3,
}

#[derive(Default, Clone, Copy)]
pub struct PPUState {
	pub cycle: u64,
	pub maxed: bool,
	pub paused: bool,
}

pub trait PPU {
	fn get_mode(&self) -> PPUMode;
	fn get_ly(&self) -> u8;
	fn set_ly(&mut self, value: u8);
	fn set_mode(&mut self, mode: PPUMode);
	fn step_ppu(&mut self);
	fn render(&mut self);
	fn get_bg_pixel(&mut self, x: u8, y: u8) -> Color;
	fn get_tile_pixel(&mut self, x: u8, y: u8, tile_index: u8, mode: bool) -> Color;
}

impl PPU for EmulatorState {
	fn get_mode(&self) -> PPUMode {
		let num = self.read(IORegistersAdress::STAT as u16) & 0b00000011;
		return match num {
			0 => PPUMode::HBlank,
			1 => PPUMode::VBlank,
			2 => PPUMode::OamScan,
			3 => PPUMode::Draw,
			_ => unreachable!(), // Since we only take the last two bits
		};
	}

	fn get_ly(&self) -> u8 {
		return self.read(IORegistersAdress::LY as u16);
	}

	fn set_ly(&mut self, value: u8) {
		let lyc_status = self.read(IORegistersAdress::LY as u16) == value;
		self.write(IORegistersAdress::LY as u16, value);
		set_bit_flag_to(self, BitFlag::Stat(STATFlag::LYCeqLY), lyc_status);

		if lyc_status && get_bit_flag(self, BitFlag::Stat(STATFlag::LYCeqLUInterruptEnable)) {
			set_bit_flag(
				self,
				BitFlag::InterruptRequest(flags::InterruptFlag::LcdStat),
			);
		}
	}

	fn set_mode(&mut self, _mode: PPUMode) {}

	fn step_ppu(&mut self) {
		self.set_ly(self.get_ly() + 1);
		self.ppu_state.paused = false;

		if self.get_ly() >= 153 {
			if self.ppu_state.maxed {
				self.set_ly(0);
				self.render();
				self.ppu_state.maxed = false;
				self.ppu_state.cycle += 908;
			} else {
				self.ppu_state.cycle += 4;
				self.ppu_state.maxed = true;
			}
		} else {
			self.ppu_state.cycle += 456;
		}

		return;
	}

	fn get_bg_pixel(&mut self, x: u8, y: u8) -> Color {
		let lcdc = self.read(0xFF40);
		let mode = lcdc & 0b00000010 == 0b00000010;

		let tile_pos = if mode { 0x8800 } else { 0x8000 };

		match (x + y) % 2 {
			0 => (0, 0, 0),
			1 => (255, 255, 255),
			_ => (255, 0, 0),
		}
	}

	fn get_tile_pixel(&mut self, x: u8, y: u8, tile_index: u8, mode: bool) -> Color {
		let (x, y) = (x as u16, y as u16);
		let addr: u16 = match mode {
			true => 0x8800_i32 + ((tile_index as i8) as i32 * 16),
			false => 0x8000_i32 + (tile_index as i32) * 16,
		} as u16;

		match (
			get_bit(self.read(addr + y * 2 + 0), x as u8),
			get_bit(self.read(addr + y * 2 + 1), x as u8),
		) {
			(true, true) => (8, 24, 32),
			(true, false) => (224, 248, 208),
			(false, true) => (52, 104, 86),
			(false, false) => (136, 192, 112),
		}
	}

	fn render(&mut self) {
		// Bit 7 - LCD Display Enable             (0=Off, 1=On)
		// Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
		// Bit 5 - Window Display Enable          (0=Off, 1=On)
		// Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
		// Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
		// Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
		// Bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)
		// Bit 0 - BG/Window Display/Priority     (0=Off, 1=On)

		let lcdc = self.read(0xFF40);

		let lcd_enable = lcdc & 0b00000001 == 0b00000001;
		let window_tile_map_display_sel = lcdc & 0b00000010 == 0b00000010;
		let window_display_enable = lcdc & 0b00000100 == 0b00000100;
		let bg_win_tile_map_sel = lcdc & 0b00001000 == 0b00001000;
		let bg_tile_map_sel = lcdc & 0b00010000 == 0b00010000;
		let sprite_size = lcdc & 0b00100000 == 0b00100000;
		let sprite_enable = lcdc & 0b01000000 == 0b01000000;
		let bg_window_priority = lcdc & 0b10000000 == 0b10000000;
		let (scx, scy) = (self.read(0xFF42), self.read(0xFF43));
		let (wx, wy) = (self.read(0xFF4B), self.read(0xFF4A));

		// $8000-$97FF Tile Data

		// Block 0 is $8000-87FF
		// Block 1 is $8800-8FFF
		// Block 2 is $9000-97FF

		for x in scx..scx + 160 {
			for y in scy..scy + 144 {
				let color = self.get_bg_pixel(x, y);
				self.lcd.put_pixel(scx - x, scy - y, color);
			}
		}

		// self.lcd.put_pixel(x, y, color);
	}
}
