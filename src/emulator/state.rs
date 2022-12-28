use serde::Serialize;

use super::{
	cartridge::{header::CartridgeParseError, memory_bank_controller::Cartridge},
	cpu::{registers::CPURegister16, values::ValueRefU16, CPUState, CPU},
	flags::INTERRUPT_REQUEST,
	io_registers::IORegisterState,
	lcd::LCD,
	memory_mapper::MemoryMapper,
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
	pub lcd: Option<LCD>,
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
			cpu_state: CPUState::default(),
			ppu_state: PPUState::default(),
			io_register_state: IORegisterState::default(),
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

		emulator.write_16(&ValueRefU16::Reg(CPURegister16::AF), 0x01B0);
		emulator.write_16(&ValueRefU16::Reg(CPURegister16::BC), 0x0013);
		emulator.write_16(&ValueRefU16::Reg(CPURegister16::DE), 0x00D8);
		emulator.write_16(&ValueRefU16::Reg(CPURegister16::HL), 0x014D);

		emulator.write_16(&ValueRefU16::Reg(CPURegister16::SP), 0xFFFE);
		emulator.write_16(&ValueRefU16::Reg(CPURegister16::PC), 0x0100);
		emulator.set_mode(PPUMode::OamScan);

		emulator.io_register_state[0xFF04] = 0x1F;
		emulator.write(0xFF10, 0x80);
		emulator.write(0xFF11, 0xBF);
		emulator.write(0xFF12, 0xF3);
		emulator.write(0xFF14, 0xBF);
		emulator.write(0xFF16, 0x3F);
		emulator.write(0xFF19, 0xBF);
		emulator.write(0xFF1A, 0x7F);
		emulator.write(0xFF1B, 0xFF);
		emulator.write(0xFF1C, 0x9F);
		emulator.write(0xFF1E, 0xBF);
		emulator.write(0xFF20, 0xFF);
		emulator.write(0xFF23, 0xBF);
		emulator.write(0xFF24, 0x77);
		emulator.write(0xFF25, 0xF3);
		emulator.write(0xFF26, 0xF1);
		emulator.write(0xFF40, 0x91);
		emulator.write(0xFF44, 0x90);
		emulator.write(0xFF47, 0xFC);
		emulator.write(0xFF48, 0xFF);
		emulator.write(0xFF49, 0xFF);

		for _ in 0..258 {
			emulator.step_ppu();
		}

		emulator
	}
}

impl EmulatorState {
	pub fn get_cycle(&self) -> u64 {
		self.t_states / 4
	}

	pub fn step(&mut self) {
		self.step_cpu();
	}

	pub fn bind_lcd(&mut self, lcd: LCD) {
		self.lcd = Some(lcd);
	}

	pub fn tick_m_cycles(&mut self, m_cycles: u64) {
		self.tick_t_states(m_cycles * 4);
	}

	fn tick_t_states(&mut self, t_states: u64) {
		for _ in 0..t_states {
			self.step_ppu();
		}
		self.update_timer(t_states);

		self.t_states += t_states;
	}

	pub fn request_interrupt(&mut self, interrupt: u8) {
		self.write(INTERRUPT_REQUEST, self.read(INTERRUPT_REQUEST) | interrupt);
	}

	pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), CartridgeParseError> {
		let cartridge = Cartridge::try_from(rom)?;
		self.cartridge_state = Some(cartridge);
		Ok(())
	}
}
