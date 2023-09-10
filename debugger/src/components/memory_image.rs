use egui::{Color32, ColorImage, Image, TextureHandle, TextureOptions, Ui, Vec2};
use gameboy::Gameboy;
use sm83::{memory_mapper::MemoryMapper, SM83};

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
impl AddressRange {
	fn color(&self) -> egui::Color32 {
		match self {
			AddressRange::RomBank0 => egui::Color32::from_rgb(186, 172, 147),
			AddressRange::RomBankN => egui::Color32::from_rgb(41, 112, 248),
			AddressRange::VRam => egui::Color32::from_rgb(193, 181, 201),
			AddressRange::ExternalRam => egui::Color32::from_rgb(197, 149, 95),
			AddressRange::WRamBank0 => egui::Color32::from_rgb(168, 254, 95),
			AddressRange::WRamBankN => egui::Color32::from_rgb(12, 175, 83),
			AddressRange::Mirror => egui::Color32::from_rgb(247, 235, 214),
			AddressRange::SpriteAttributes => egui::Color32::from_rgb(228, 188, 134),
			AddressRange::Unusable => egui::Color32::from_rgb(71, 67, 135),
			AddressRange::IORegisters => egui::Color32::from_rgb(63, 69, 11),
			AddressRange::HighRam => egui::Color32::from_rgb(125, 27, 237),
			AddressRange::InterruptEnable => egui::Color32::from_rgb(174, 119, 138),
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
			res.image.pixels[i as usize] = AddressRange::from(i).color();
		}

		res
	}
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
			self.image.pixels[i as usize] = AddressRange::from(i)
				.color()
				.linear_multiply(gameboy.memory_mapper().read(i) as f32 / 255.0);
		}
		self.image.pixels[gameboy.cpu_state.registers.pc as usize] = Color32::RED;

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
