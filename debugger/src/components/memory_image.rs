use egui::{Color32, ColorImage, Image, Rgba, TextureHandle, TextureOptions, Ui, Vec2};
use gameboy::Gameboy;
use sm83::{
	memory_mapper::MemoryMapper,
	registers::{Addressable, CPURegister16},
};

pub struct MemoryImage {
	texture: Option<TextureHandle>,
	image: ColorImage,
}

#[derive(Debug)]
enum AddressRange {
	RomBank0,
	RomBankN,
	VRam,
	ExternalRam,
	WRamBank0,
	WRamBankN,
	Mirror,
	SpriteAttributes,
	Unusable,
	IORegisters,
	HighRam,
	InterruptEnable,
}

const COLORS: [egui::Rgba; 12] = [
	egui::Rgba::from_rgba_premultiplied(186.0 / 255.0, 172.0 / 255.0, 147.0 / 255.0, 255.0),
	egui::Rgba::from_rgba_premultiplied(41.0 / 255.0, 112.0 / 255.0, 248.0 / 255.0, 255.0),
	egui::Rgba::from_rgba_premultiplied(193.0 / 255.0, 181.0 / 255.0, 201.0 / 255.0, 255.0),
	egui::Rgba::from_rgba_premultiplied(197.0 / 255.0, 149.0 / 255.0, 95.0 / 255.0, 255.0),
	egui::Rgba::from_rgba_premultiplied(168.0 / 255.0, 254.0 / 255.0, 95.0 / 255.0, 255.0),
	egui::Rgba::from_rgba_premultiplied(12.0 / 255.0, 175.0 / 255.0, 83.0 / 255.0, 255.0),
	egui::Rgba::from_rgba_premultiplied(247.0 / 255.0, 235.0 / 255.0, 214.0 / 255.0, 255.0),
	egui::Rgba::from_rgba_premultiplied(228.0 / 255.0, 188.0 / 255.0, 134.0 / 255.0, 255.0),
	egui::Rgba::from_rgba_premultiplied(71.0 / 255.0, 67.0 / 255.0, 135.0 / 255.0, 255.0),
	egui::Rgba::from_rgba_premultiplied(63.0 / 255.0, 69.0 / 255.0, 11.0 / 255.0, 255.0),
	egui::Rgba::from_rgba_premultiplied(125.0 / 255.0, 27.0 / 255.0, 237.0 / 255.0, 255.0),
	egui::Rgba::from_rgba_premultiplied(174.0 / 255.0, 119.0 / 255.0, 138.0 / 255.0, 255.0),
];

impl AddressRange {
	#[inline(always)]
	const fn color(&self) -> egui::Rgba {
		match self {
			AddressRange::RomBank0 => COLORS[0],
			AddressRange::RomBankN => COLORS[1],
			AddressRange::VRam => COLORS[2],
			AddressRange::ExternalRam => COLORS[3],
			AddressRange::WRamBank0 => COLORS[4],
			AddressRange::WRamBankN => COLORS[5],
			AddressRange::Mirror => COLORS[6],
			AddressRange::SpriteAttributes => COLORS[7],
			AddressRange::Unusable => COLORS[8],
			AddressRange::IORegisters => COLORS[9],
			AddressRange::HighRam => COLORS[10],
			AddressRange::InterruptEnable => COLORS[11],
		}
	}
}

impl From<u16> for AddressRange {
	fn from(addr: u16) -> Self {
		match addr {
			0x0000..0x4000 => Self::RomBank0,
			0x4000..0x8000 => Self::RomBankN,
			0x8000..0xA000 => Self::VRam,
			0xA000..0xC000 => Self::ExternalRam,
			0xC000..0xD000 => Self::WRamBank0,
			0xD000..0xE000 => Self::WRamBankN,
			0xE000..0xFE00 => Self::Mirror,
			0xFE00..0xFEA0 => Self::SpriteAttributes,
			0xFEA0..0xFF00 => Self::Unusable,
			0xFF04..0xFF10 => Self::IORegisters,
			0xFF00..0xFF80 => Self::IORegisters,
			0xFF80..0xFFFE => Self::HighRam,
			_ => Self::InterruptEnable,
		}
	}
}

impl Default for MemoryImage {
	fn default() -> Self {
		let mut res = Self {
			image: ColorImage::new([256, 256], egui::Color32::from_rgb(186, 172, 147)),
			texture: None,
		};

		for i in 0..u16::MAX {
			res.image.pixels[i as usize] = AddressRange::from(i).color().into();
		}

		res
	}
}

fn cheap_multiply_color(color: Rgba, factor: f32) -> Color32 {
	let [r, g, b, a] = color.to_array();

	let factor = factor * 255.0;
	Color32::from_rgba_premultiplied(
		(r * factor) as u8,
		(g * factor) as u8,
		(b * factor) as u8,
		(a * factor) as u8,
	)
}

impl MemoryImage {
	fn legend(ui: &mut Ui) {
		fn row(ui: &mut Ui, range: AddressRange) {
			ui.colored_label(range.color(), format!("{range:?}"));
		}
		ui.vertical(|ui| {
			row(ui, AddressRange::RomBank0);
			row(ui, AddressRange::RomBankN);
			row(ui, AddressRange::VRam);
			row(ui, AddressRange::ExternalRam);
			row(ui, AddressRange::WRamBank0);
			row(ui, AddressRange::WRamBankN);
			row(ui, AddressRange::Mirror);
			row(ui, AddressRange::SpriteAttributes);
			row(ui, AddressRange::Unusable);
			row(ui, AddressRange::IORegisters);
			row(ui, AddressRange::HighRam);
			row(ui, AddressRange::InterruptEnable);
		});
	}

	pub fn render_img(&mut self, gameboy: &Gameboy) {
		for i in 0..u16::MAX {
			let color = AddressRange::from(i).color();
			let opacity = gameboy.read(i) as f32 / 255.0;
			self.image.pixels[i as usize] = cheap_multiply_color(color, opacity);
		}
		self.image.pixels[gameboy.cpu_state.read(CPURegister16::PC) as usize] = Color32::RED;

		let hdma5 = gameboy.dma_controller.read_hdma5();

		if hdma5 & 0b10000000 == 0 {
			let source = gameboy.dma_controller.get_source();
			let destination = gameboy.dma_controller.get_destination();
			for i in 0..16 {
				self.image.pixels[(source + i) as usize] = Color32::GOLD;
				self.image.pixels[(destination + i) as usize] = Color32::LIGHT_RED;
			}
		}
	}

	pub fn draw(&mut self, gameboy: &Gameboy, ui: &mut Ui) {
		self.render_img(gameboy);

		let texture = self.texture.get_or_insert_with(|| {
			ui.ctx()
				.load_texture("screen", self.image.clone(), TextureOptions::NEAREST)
		});

		texture.set(self.image.clone(), TextureOptions::NEAREST);

		ui.horizontal(|ui| {
			ui.vertical(MemoryImage::legend);
			ui.add(Image::new(
				texture.id(),
				Vec2::new(160.0 * 4.0, 144.0 * 4.0),
			));
		});
	}
}
