use serde::{Deserialize, Serialize};

use crate::{
	cgb::{CGBState, Speed},
	dma_controller::{DMAController, TransferRequest},
	lcd::Color,
	oam_dma::OamDmaState,
	ppu::VRAMBank,
	util::BigArray,
	work_ram::{BankedWorkRam, WorkRam, WorkRamDataCGB, WorkRamDataDMG},
};

use super::{
	cartridge::memory_bank_controller::Cartridge,
	controller::ControllerState,
	io_registers::IORegisterState,
	ppu::{PPUMode, PPU},
	save_state::{RomSource, SaveState},
	timer::Timer,
};

use sm83::{memory_mapper::MemoryMapper, CPUState, SM83};

#[derive(Clone, Serialize, Deserialize)]
pub enum Mode {
	// Gameboy Color mode
	GBC(CGBState),
	// Basic monochrome mode
	DMG,
}
const DMG_SPEED: Speed = Speed::Normal;

impl Mode {
	pub fn get_speed(&self) -> &Speed {
		match self {
			Mode::GBC(state) => state.current_speed(),
			Mode::DMG => &DMG_SPEED,
		}
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Gameboy {
	pub ram_bank: u8,
	pub mode: Mode,
	pub cpu_state: CPUState,
	pub ppu: PPU,
	pub cartridge_state: Option<Cartridge>,
	pub w_ram: WorkRam,

	#[serde(with = "BigArray")]
	pub hram: [u8; 0x80],
	pub io_register_state: IORegisterState,
	pub serial_output: Vec<u8>,
	pub timer: Timer,
	pub raw_joyp_input: u8,
	pub booting: bool,
	pub boot_rom: Vec<u8>,
	pub dma_controller: DMAController,
	pub oam_dma: OamDmaState,
	t_states: u64,
	pub color_scheme_dmg: (Color, Color, Color, Color),
	pub speed_switch_delay: u32,
}

impl Default for Gameboy {
	fn default() -> Self {
		let mut cpu_state = CPUState::default();

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
			ppu: PPU::default(),
			timer: Timer::default(),
			io_register_state: IORegisterState::default(),
			boot_rom: include_bytes!("../../roms/other/dmg_boot.bin").to_vec(),
			booting: true,
			cartridge_state: None,
			ram_bank: 0,
			mode: Mode::DMG,
			w_ram: WorkRam::Dmg(Box::<WorkRamDataDMG>::default()),
			hram: [0; 0x80],
			serial_output: vec![],
			raw_joyp_input: 0,
			t_states: 0,
			speed_switch_delay: 0,
		};
		emulator.set_gb_mode(Mode::GBC(CGBState::default()));
		emulator
			.ppu
			.set_mode(PPUMode::OamScan, &mut cpu_state.interrupt_request);
		emulator
	}
}

impl Gameboy {
	pub fn cgb() -> Self {
		let mut state = Self::default();
		state.set_gb_mode(Mode::GBC(Default::default()));
		state
	}

	pub fn dmg() -> Self {
		let mut state = Self::default();
		state.set_gb_mode(Mode::DMG);
		state
	}

	pub fn get_cycle(&self) -> u64 {
		self.t_states / 4
	}

	pub fn run_until_boot(&mut self) {
		while self.booting {
			self.step_cpu();
		}
	}

	pub fn step(&mut self) {
		if self.speed_switch_delay > 0 {
			self.speed_switch_delay = self.speed_switch_delay.saturating_sub(1);
			self.tick_m_cycles(1);
			return;
		}
		self.step_cpu();
	}

	pub fn tick_m_cycles(&mut self, m_cycles: u32) {
		for _ in 0..m_cycles {
			self.oam_dma.step(1, &mut self.ppu);

			let t_states = match self.mode.get_speed() {
				Speed::Normal => 4,
				Speed::Double => 2,
			};

			let new_ppu_mode = self.tick_t_states(t_states);

			if matches!(new_ppu_mode, Some(PPUMode::HBlank)) {
				if let Some(request) = self.dma_controller.step() {
					self.handle_transfer(request)
				}
			}

			if self.speed_switch_delay == 0 {
				self.timer.step(
					1,
					self.mode.get_speed(),
					&mut self.cpu_state.interrupt_request,
				);
			}
		}
	}

	pub fn handle_transfer(&mut self, request: TransferRequest) {
		log::info!("[{:X}]:{request:?}", self.get_cycle());
		let TransferRequest { from, to, bytes } = request;
		for i in 0..bytes {
			self.write(to + i, self.read(from + i));
		}
	}

	fn tick_t_states(&mut self, t_states: u32) -> Option<PPUMode> {
		let mut new_mode: Option<PPUMode> = None;

		for _ in 0..t_states {
			let mode = self.ppu.step_ppu(&mut self.cpu_state.interrupt_request);
			if let Some(mode) = mode {
				new_mode = Some(mode);
			};
		}

		self.t_states += t_states as u64;
		new_mode
	}

	pub fn request_interrupt(&mut self, interrupt: u8) {
		self.cpu_state.interrupt_request |= interrupt;
	}

	pub fn load_rom(&mut self, rom: &[u8], source: Option<RomSource>) {
		if let Ok(cart) = Cartridge::try_new(rom, source) {
			self.cartridge_state = Some(cart);
		}
	}

	pub fn set_controller_state(&mut self, state: &ControllerState) {
		self.raw_joyp_input = state.as_byte();

		if ((self.raw_joyp_input) ^ state.as_byte()) & state.as_byte() != 0 {
			self.request_interrupt(0b10000);
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

	fn set_gb_mode(&mut self, mode: Mode) {
		self.boot_rom = match mode {
			Mode::DMG => include_bytes!("../../roms/other/dmg_boot.bin").to_vec(),
			Mode::GBC(_) => include_bytes!("../../roms/other/cgb_boot.bin").to_vec(),
		};

		self.w_ram = match mode {
			Mode::GBC(_) => WorkRam::Cgb(Box::<WorkRamDataCGB>::default()),
			Mode::DMG => WorkRam::Dmg(Box::<WorkRamDataDMG>::default()),
		};
		self.mode = mode;
	}

	pub fn get_wram_bank(&self) -> usize {
		self.w_ram.get_bank_number() as usize
	}

	pub fn get_vram_bank(&self) -> VRAMBank {
		match &self.mode {
			Mode::DMG => VRAMBank::Bank0,
			Mode::GBC(state) => state.get_vram_bank(),
		}
	}
}

impl SM83<Gameboy> for Gameboy {
	fn get_memory_mapper_mut(&mut self) -> &mut Gameboy {
		self
	}

	fn get_memory_mapper(&self) -> &Gameboy {
		self
	}

	fn cpu_state(&self) -> &CPUState {
		&self.cpu_state
	}

	fn cpu_state_mut(&mut self) -> &mut CPUState {
		&mut self.cpu_state
	}

	fn debug(&self) -> bool {
		false
	}
	fn on_m_cycle(&mut self, m_cycles: u32) {
		Gameboy::tick_m_cycles(self, m_cycles)
	}

	fn exec_stop(&mut self) {
		if let crate::state::Mode::GBC(state) = &mut self.mode {
			self.speed_switch_delay = 2050;
			state.perform_speed_switch();
		}
	}
}
