use serde::{Deserialize, Serialize};

use crate::{
	cgb::{CGBState, Speed},
	dma_controller::{DMAController, TransferMode},
	oam_dma::OamDmaState,
	ppu::VRAMBank,
	util::BigArray,
	work_ram::{BankedWorkRam, WorkRam, WorkRamDataCGB, WorkRamDataDMG},
};

use super::{
	cartridge::{header::CartridgeParseError, memory_bank_controller::Cartridge},
	controller::ControllerState,
	cpu::{CPUState, CPU},
	flags::{INTERRUPT_REQUEST, INT_JOY_PAD},
	io_registers::IORegisterState,
	lcd::LCD,
	memory_mapper::{Source, SourcedMemoryMapper},
	ppu::{PPUMode, PPU},
	save_state::{RomSource, SaveState},
	timer::Timer,
};

type Color = (u8, u8, u8, u8);

#[derive(Clone, Serialize, Deserialize)]
pub enum GameboyMode {
	// Gameboy Color mode
	GBC(CGBState),
	// Basic monochrome mode
	DMG,
}

impl GameboyMode {
	pub fn get_speed(&self) -> Speed {
		match self {
			GameboyMode::GBC(state) => state.current_speed(),
			GameboyMode::DMG => Speed::Normal,
		}
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Gameboy {
	pub ram_bank: u8,
	pub mode: GameboyMode,
	pub cpu_state: CPUState,
	pub ppu: PPU,
	pub cartridge_state: Option<Cartridge>,
	pub w_ram: WorkRam,

	#[serde(with = "BigArray")]
	pub hram: [u8; 0x80],
	pub io_register_state: IORegisterState,
	pub serial_output: Vec<u8>,
	pub timer: Timer,
	pub halted: bool,
	pub interrupt_enable_register: u8,
	pub raw_joyp_input: u8,
	pub booting: bool,
	pub boot_rom: Vec<u8>,
	pub dma_controller: DMAController,
	pub oam_dma: OamDmaState,
	t_states: u64,
	pub next_interrupt: Option<u8>,
	pub color_scheme_dmg: (Color, Color, Color, Color),
}

impl Default for Gameboy {
	fn default() -> Self {
		let mut emulator = Self {
			dma_controller: DMAController::default(),
			color_scheme_dmg: (
				(0xFF, 0xFF, 0xFF, 0xFF),
				(0xAA, 0xAA, 0xAA, 0xFF),
				(0x55, 0x55, 0x55, 0xFF),
				(0x00, 0x00, 0x00, 0xFF),
			),
			oam_dma: Default::default(),
			cpu_state: CPUState::default(),
			ppu: PPU::new(),
			timer: Timer::default(),
			io_register_state: IORegisterState::default(),
			boot_rom: include_bytes!("../../roms/other/dmg_boot.bin").to_vec(),
			booting: true,
			cartridge_state: None,
			ram_bank: 0,
			mode: GameboyMode::DMG,
			w_ram: WorkRam::DMG(WorkRamDataDMG::default()),
			hram: [0; 0x80],
			serial_output: vec![],
			halted: false,
			interrupt_enable_register: 0,
			raw_joyp_input: 0,
			t_states: 0,
			next_interrupt: None,
		};
		emulator.set_gb_mode(GameboyMode::GBC(CGBState::default()));
		emulator.ppu.set_mode(PPUMode::OamScan);
		emulator
	}
}

impl Gameboy {
	pub fn get_cycle(&self) -> u64 {
		self.t_states / 4
	}

	pub fn run_until_boot(&mut self) {
		while self.booting {
			self.step_cpu();
		}
	}

	pub fn step(&mut self) {
		self.step_cpu();
	}

	pub fn bind_lcd(&mut self, lcd: LCD) {
		self.ppu.lcd = Some(lcd);
	}

	pub fn tick_m_cycles(&mut self, m_cycles: u32) {
		self.oam_dma.step(m_cycles, &mut self.ppu);
		match self.mode.get_speed() {
			Speed::Normal => self.tick_t_states(m_cycles * 4),
			Speed::Double => self.tick_t_states(m_cycles * 2),
		}
	}

	fn tick_t_states(&mut self, t_states: u32) {
		let current_bank = &self.get_vram_bank();

		let do_dma = if let Some(transfer) = &self.dma_controller.transfer {
			matches!(transfer.mode, TransferMode::GeneralPurpose)
		} else {
			false
		};

		if do_dma {
			self.dma_controller.step_controller(
				&mut self.ppu,
				&mut self.cartridge_state,
				&mut self.w_ram,
				current_bank,
			);
		}

		for _ in 0..t_states {
			match self.ppu.step_ppu() {
				Some(PPUMode::HBlank) => {
					self.dma_controller.step_controller(
						&mut self.ppu,
						&mut self.cartridge_state,
						&mut self.w_ram,
						current_bank,
					);
				}
				_ => {}
			}
		}

		self.timer.step(t_states as u64, self.mode.get_speed());

		self.t_states += t_states as u64;
	}

	pub fn request_interrupt(&mut self, interrupt: u8) {
		self.write_from(
			INTERRUPT_REQUEST,
			self.read_from(INTERRUPT_REQUEST, Source::Raw) | interrupt,
			Source::Raw,
		);
	}

	pub fn load_rom(
		&mut self,
		rom: &[u8],
		source: Option<RomSource>,
	) -> Result<(), CartridgeParseError> {
		let cartridge = Cartridge::try_new(rom, source)?;
		self.cartridge_state = Some(cartridge);
		Ok(())
	}

	pub fn set_controller_state(&mut self, state: &ControllerState) {
		self.raw_joyp_input = state.as_byte();

		if ((self.raw_joyp_input) ^ state.as_byte()) & state.as_byte() != 0 {
			self.request_interrupt(INT_JOY_PAD);
		}
	}

	pub fn load_save_state(self, save_state: SaveState) -> Self {
		let Some(cart) = self.cartridge_state.as_ref() else {
			return self;
		};

		let Ok(mut new_state) = serde_json::from_str::<Gameboy>(&save_state.data) else {
			return self;
		};

		let Some(new_cart) = new_state.cartridge_state.as_mut() else {
			return self;
		};

		let Cartridge(data, _, info) = cart;

		if info.title != new_cart.2.title {
			return self;
		}

		new_cart.0.rom_banks = data.rom_banks.clone();
		new_cart.0.loaded = true;

		new_state
	}

	fn set_gb_mode(&mut self, mode: GameboyMode) {
		self.boot_rom = match mode {
			GameboyMode::DMG => include_bytes!("../../roms/other/dmg_boot.bin").to_vec(),
			GameboyMode::GBC(_) => include_bytes!("../../roms/other/cgb_boot.bin").to_vec(),
		};

		self.w_ram = match mode {
			GameboyMode::GBC(_) => WorkRam::CGB(WorkRamDataCGB::default()),
			GameboyMode::DMG => WorkRam::DMG(WorkRamDataDMG::default()),
		};
		self.mode = mode;
	}

	pub fn get_wram_bank(&self) -> usize {
		self.w_ram.get_bank_number() as usize
	}

	pub fn get_vram_bank(&self) -> VRAMBank {
		match &self.mode {
			GameboyMode::DMG => VRAMBank::Bank0,
			GameboyMode::GBC(state) => state.get_vram_bank(),
		}
	}
}
