use instant::Instant;
use serde::Serialize;

use super::{
	cartridge::{header::CartridgeParseError, memory_bank_controller::Cartridge},
	cpu::{CPUState, CPU},
	flags::INTERRUPT_REQUEST,
	io_registers::IORegisterState,
	lcd::LCD,
	memory_mapper::{Source, SourcedMemoryMapper},
	ppu::{PPUMode, PPUState, PPU},
	timer::{Timer, TimerState},
};

#[derive(Clone, Serialize)]
pub struct EmulatorState {
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
	pub lcd: Option<LCD>,
	pub booting: bool,
	pub boot_rom: &'static [u8],
	t_states: u64,
}

impl PPUState {
	pub fn pause(&mut self) {
		if !self.paused {
			self.paused = true;
			self.cycle += 190000 - (5080 - 6);
		}
	}
}

impl Default for EmulatorState {
	fn default() -> Self {
		let mut emulator = Self {
			dma_timer: 0,
			cpu_state: CPUState::default(),
			ppu_state: PPUState::default(),
			io_register_state: IORegisterState::default(),
			boot_rom: include_bytes!("../../roms/other/dmg_boot.bin"),
			booting: true,
			cartridge_state: None,
			ram_bank: 0,
			cgb: false,
			v_ram: vec![vec![0; 0x2000]; 2],
			w_ram: vec![vec![0; 0x1000]; 8],
			oam: vec![0; 0xA0],
			hram: vec![0; 0x80],
			serial_output: vec![],
			timer_state: TimerState::default(),
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

impl EmulatorState {
	pub fn get_cycle(&self) -> u64 {
		self.t_states / 4
	}

	pub fn run_until_boot(&mut self) {
		let start = Instant::now();
		while self.booting {
			self.step_cpu();
		}
		log::warn!("BIOS took: {:?}", Instant::now().duration_since(start))
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

	pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), CartridgeParseError> {
		let cartridge = Cartridge::try_from(rom)?;
		self.cartridge_state = Some(cartridge);
		self.run_until_boot();
		Ok(())
	}
}
