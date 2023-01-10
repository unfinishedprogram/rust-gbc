use serde::{Deserialize, Serialize};

use super::{
	cartridge::{header::CartridgeParseError, memory_bank_controller::Cartridge},
	controller::ControllerState,
	cpu::{CPUState, CPU},
	flags::{INTERRUPT_REQUEST, INT_JOY_PAD},
	io_registers::IORegisterState,
	lcd::LCD,
	memory_mapper::{Source, SourcedMemoryMapper},
	ppu::{PPUMode, PPUState, PPU},
	save_state::{RomSource, SaveState},
	timer::{Timer, TimerState},
};

type Color = (u8, u8, u8, u8);

#[derive(Clone, Serialize, Deserialize)]
pub struct Gameboy {
	pub ram_bank: u8,
	pub cgb: bool,
	pub cpu_state: CPUState,
	pub ppu_state: PPUState,
	pub cartridge_state: Option<Cartridge>,
	pub v_ram: Vec<Vec<u8>>,
	pub w_ram: Vec<Vec<u8>>,
	pub oam: Vec<u8>,
	pub hram: Vec<u8>,
	pub io_register_state: IORegisterState,
	pub serial_output: Vec<u8>,
	pub timer_state: TimerState,
	pub halted: bool,
	pub interrupt_enable_register: u8,
	pub raw_joyp_input: u8,
	pub dma_timer: u64,
	#[serde(skip)]
	pub lcd: Option<LCD>,
	pub booting: bool,
	pub boot_rom: Vec<u8>,
	t_states: u64,
	pub color_scheme_dmg: (Color, Color, Color, Color),
}

impl Default for Gameboy {
	fn default() -> Self {
		let mut emulator = Self {
			color_scheme_dmg: (
				(0xFF, 0xFF, 0xFF, 0xFF),
				(0xAA, 0xAA, 0xAA, 0xFF),
				(0x55, 0x55, 0x55, 0xFF),
				(0x00, 0x00, 0x00, 0xFF),
			),
			dma_timer: 0,
			cpu_state: CPUState::default(),
			ppu_state: PPUState::default(),
			timer_state: TimerState::default(),
			io_register_state: IORegisterState::default(),
			boot_rom: include_bytes!("./dmg_boot.bin").to_vec(),
			booting: true,
			cartridge_state: None,
			ram_bank: 0,
			cgb: false,
			v_ram: vec![vec![0; 0x2000]; 2],
			w_ram: vec![vec![0; 0x1000]; 8],
			oam: vec![0; 0xA0],
			hram: vec![0; 0x80],
			serial_output: vec![],
			halted: false,
			interrupt_enable_register: 0,
			raw_joyp_input: 0,
			lcd: None,
			t_states: 0,
		};
		emulator.set_mode(PPUMode::OamScan);
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
		self.lcd = Some(lcd);
	}

	pub fn tick_m_cycles(&mut self, m_cycles: u32) {
		self.tick_t_states(m_cycles * 4);
	}

	fn tick_t_states(&mut self, t_states: u32) {
		for _ in 0..t_states {
			self.step_ppu();
		}

		self.dma_timer = self.dma_timer.saturating_sub(t_states as u64);

		self.update_timer(t_states as u64);

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
}
