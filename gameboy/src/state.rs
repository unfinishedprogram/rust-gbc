use serde::{Deserialize, Serialize};

use crate::{
	cgb::{CGBState, Speed},
	dma_controller::{DMAController, TransferRequest},
	io_registers::JOYP,
	lcd::Color,
	oam_dma::{step_oam_dma, OamDmaState},
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

use sm83::{memory_mapper::MemoryMapper, CPUState, Instruction, Interrupt, SM83};

#[derive(Clone, Serialize, Deserialize)]
pub enum Mode {
	// Gameboy Color mode
	GBC(CGBState),
	// Basic monochrome mode
	DMG,
}

impl Mode {
	pub fn get_speed(&self) -> Speed {
		match self {
			Mode::GBC(state) => state.current_speed(),
			Mode::DMG => Speed::Normal,
		}
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Gameboy {
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
	pub t_states: u64,
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

	pub fn run_until_boot(&mut self) {
		while self.booting {
			self.step_cpu();
		}
	}

	pub fn step(&mut self) -> Option<Instruction> {
		if self.speed_switch_delay > 0 {
			self.tick_m_cycles(1);
			return None;
		}
		self.step_cpu()
	}

	pub fn tick_m_cycles(&mut self, m_cycles: u32) {
		for _ in 0..m_cycles {
			self.speed_switch_delay = self.speed_switch_delay.saturating_sub(1);
			step_oam_dma(self);

			let t_states = match self.mode.get_speed() {
				Speed::Normal => 4,
				Speed::Double => 2,
			};

			self.tick_t_states(t_states);

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
		log::info!("[{:X}]:{request:?}", self.t_states / 4);
		let TransferRequest { from, to, bytes } = request;
		for i in 0..bytes {
			self.write(to + i, self.read(from + i));
		}

		let speed_div = match self.mode.get_speed() {
			Speed::Normal => 2,
			Speed::Double => 1,
		};

		self.tick_m_cycles(bytes as u32 / speed_div);
	}

	fn tick_t_states(&mut self, t_states: u32) {
		for _ in 0..t_states {
			let mode = self.ppu.step_ppu(&mut self.cpu_state.interrupt_request);
			if matches!(mode, Some(PPUMode::HBlank)) {
				if let Some(request) = self.dma_controller.step() {
					self.handle_transfer(request)
				}
			}
		}

		self.t_states += t_states as u64;
	}

	pub fn request_interrupt(&mut self, interrupt: Interrupt) {
		self.cpu_state.interrupt_request |= interrupt.flag_bit();
	}

	pub fn load_rom(&mut self, rom: &[u8], source: Option<RomSource>) {
		if let Ok(cart) = Cartridge::try_new(rom, source) {
			self.cartridge_state = Some(cart);
		}
	}

	pub fn set_controller_state(&mut self, state: &ControllerState) {
		self.raw_joyp_input = state.as_byte();

		if ((self.raw_joyp_input) ^ state.as_byte()) & state.as_byte() != 0 {
			self.request_interrupt(Interrupt::JoyPad);
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
	fn memory_mapper_mut(&mut self) -> &mut Gameboy {
		self
	}

	fn memory_mapper(&self) -> &Gameboy {
		self
	}

	fn cpu_state(&self) -> &CPUState {
		&self.cpu_state
	}

	fn cpu_state_mut(&mut self) -> &mut CPUState {
		&mut self.cpu_state
	}

	fn on_m_cycle(&mut self, m_cycles: u32) {
		Gameboy::tick_m_cycles(self, m_cycles)
	}

	fn exec_stop(&mut self) {
		// https://gbdev.io/pandocs/Reducing_Power_Consumption.html?highlight=stop#using-the-stop-instruction

		let interrupt_pending = self.cpu_state.interrupt_pending();
		let has_joyp_input = self.read(JOYP) & 0b1111 != 0;
		let Some(speed_switch_pending) = (match &self.mode {
			Mode::GBC(state) => Some(state.prepare_speed_switch),
			Mode::DMG => None,
		}) else {
			return;
		};

		if has_joyp_input {
			if interrupt_pending {
				// STOP is a 1 byte opcode
				// mode does not change
				// DIV does not reset
			} else {
				// STOP is a 1 byte opcode
				// HALT mode is entered
				// DIV is reset
				self.timer.set_div(0, self.mode.get_speed());
				self.cpu_state.halted = true;
			}
		} else if speed_switch_pending {
			if !interrupt_pending {
				self.next_byte();
			}

			self.timer.set_div(0, self.mode.get_speed());

			let switched = if let Mode::GBC(state) = &mut self.mode {
				state.perform_speed_switch()
			} else {
				false
			};
			if switched {
				self.speed_switch_delay = 2050;
				// self.speed_switch_delay = 32 * 1024;
				// self.speed_switch_delay = 1025 * 2;
			}
		} else {
			if !interrupt_pending {
				self.next_byte();
			}
			self.timer.set_div(0, self.mode.get_speed());
		}
	}
}
