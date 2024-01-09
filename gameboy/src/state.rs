use serde::{Deserialize, Serialize};

use crate::{
	cgb::{CGBState, Speed},
	dma_controller::{DMAController, DMATransferRequest},
	io_registers::JOYP,
	oam_dma::{step_oam_dma, OamDmaState},
	ppu::{self, VRAMBank},
	util::BigArray,
	work_ram::{BankedWorkRam, WorkRam, WorkRamDataCGB, WorkRamDataDMG},
};

use super::{
	cartridge::memory_bank_controller::Cartridge,
	io_registers::IORegisterState,
	joypad::JoypadState,
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
	debug_break: bool,
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
	pub speed_switch_delay: u32,
}

impl Default for Gameboy {
	fn default() -> Self {
		let mut cpu_state = CPUState::default();

		let mut emulator: Gameboy = Self {
			debug_break: false,
			dma_controller: DMAController::default(),
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
			raw_joyp_input: 0xFF,
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

	// Steps a single cycle
	pub fn step(&mut self) -> Option<Instruction> {
		if self.speed_switch_delay > 0 {
			self.tick_m_cycles(1);
			None
		} else {
			self.step_cpu()
		}
	}

	pub fn tick_m_cycles(&mut self, m_cycles: u32) {
		for _ in 0..m_cycles {
			self.speed_switch_delay = self.speed_switch_delay.saturating_sub(1);

			let t_states: u32 = match (self.mode.get_speed(), self.speed_switch_delay) {
				(Speed::Double, 0) => 2,
				(Speed::Normal, _) => 4,
				(_, _) => 4,
			};

			self.tick_t_states(t_states);
			step_oam_dma(self);

			// Only step the timer if we aren't in a speed switch
			if self.speed_switch_delay == 0 {
				self.timer.step(&mut self.cpu_state.interrupt_request);
			}
		}
	}

	pub fn handle_dma_transfer(&mut self, request: DMATransferRequest) {
		log::info!("[{:X}]:{request:?}", self.t_states / 4);
		let DMATransferRequest {
			from,
			to,
			rows,
			gdma,
		} = request;

		// Preparation time, same in both speeds
		if gdma {
			self.tick_t_states(2);
		}

		let mut src = from;
		let mut dest = to;

		let speed_mul = match self.mode.get_speed() {
			Speed::Normal => 1,
			Speed::Double => 2,
		};

		for _ in 0..rows {
			for j in 0..16 {
				self.write(dest + j, self.read(src + j));
				if j & 1 == 1 {
					self.tick_m_cycles(speed_mul);
				}
			}
			src += 16;
			dest += 16;
		}
	}

	fn tick_t_states(&mut self, t_states: u32) {
		for _ in 0..t_states {
			let mode = self.ppu.step(&mut self.cpu_state.interrupt_request);
			if let Some(PPUMode::HBlank) = mode {
				// HDMA is not processed during speed switch
				if !self.speed_switch_delay > 0 {
					if let Some(request) = self.dma_controller.step() {
						self.handle_dma_transfer(request)
					}
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
			match cart.2.cgb {
				true => self.set_gb_mode(Mode::GBC(CGBState::default())),
				false => self.set_gb_mode(Mode::DMG),
			}
			self.cartridge_state = Some(cart);
		}
	}

	pub fn set_controller_state(&mut self, state: &JoypadState) {
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

		self.ppu.gb_mode = match mode {
			Mode::GBC(_) => ppu::GBMode::CGB,
			Mode::DMG => ppu::GBMode::DMG,
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

	pub fn debug_break(&mut self) {
		self.debug_break = true;
	}

	pub fn consume_debug_break(&mut self) -> bool {
		if self.debug_break {
			self.debug_break = false;
			return true;
		}
		false
	}
}

impl SM83 for Gameboy {
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

		let speed_switch_pending = match &self.mode {
			Mode::GBC(state) => state.prepare_speed_switch,
			Mode::DMG => false,
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
				self.timer.set_div(0);
				self.cpu_state.halted = true;
			}
		} else if speed_switch_pending {
			if !interrupt_pending {
				self.next_byte();
			}

			self.timer.set_div(0);

			let switched = if let Mode::GBC(state) = &mut self.mode {
				state.perform_speed_switch()
			} else {
				false
			};
			if switched {
				self.speed_switch_delay = 128 * 1024 / 4;
			}
		} else {
			if !interrupt_pending {
				self.next_byte();
			}
			self.timer.set_div(0);
		}
	}
}
