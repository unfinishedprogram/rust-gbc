use super::cartridge::header::CartridgeParseError;
use super::cartridge::CartridgeState;
use super::cpu::registers::CPURegister16;
use super::cpu::values::ValueRefU16;
use super::cpu::{CPUState, CPU};
use super::io_registers::IORegisterState;
use super::lcd::LCDDisplay;
use super::memory_mapper::MemoryMapper;
use super::ppu::{PPUMode, PPUState, PPU};
use super::timer_controller::TimerController;

trait LCDDisplayWithCopy: LCDDisplay + Copy {}

#[derive(Clone)]
pub struct EmulatorState {
	pub ram_bank: u8,
	pub cgb: bool,
	pub cpu_state: CPUState,
	pub ppu_state: PPUState,
	pub cartridge_state: Option<CartridgeState>,
	pub v_ram: [[u8; 0x2000]; 2],
	pub w_ram: [[u8; 0x1000]; 8],
	pub oam: [u8; 0xA0],
	pub hram: [u8; 0x80],
	pub io_register_state: IORegisterState,
	pub cycle: u64,
	pub serial_output: Vec<u8>,
	pub timer_clock: u64,
	pub div_clock: u64,
	pub halted: bool,
	pub interrupt_enable_register: u8,
	pub raw_joyp_input: u8,
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
			v_ram: [[0; 0x2000]; 2],
			w_ram: [[0; 0x1000]; 8],
			oam: [0; 0xA0],
			hram: [0; 0x80],
			cycle: 0,
			serial_output: vec![],
			timer_clock: 0,
			div_clock: 0,
			halted: false,
			interrupt_enable_register: 0,
			raw_joyp_input: 0,
		};

		emulator.write_16(ValueRefU16::Reg(CPURegister16::AF), 0x01B0);
		emulator.write_16(ValueRefU16::Reg(CPURegister16::BC), 0x0013);
		emulator.write_16(ValueRefU16::Reg(CPURegister16::DE), 0x00D8);
		emulator.write_16(ValueRefU16::Reg(CPURegister16::HL), 0x014D);

		emulator.write_16(ValueRefU16::Reg(CPURegister16::SP), 0xFFFE);
		emulator.write_16(ValueRefU16::Reg(CPURegister16::PC), 0x0100);
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

		emulator.ppu_state.cycle = 512;
		emulator
	}
}

impl EmulatorState {
	pub fn step(&mut self, lcd: &mut dyn LCDDisplay) {
		while self.cycle * 16 > self.ppu_state.cycle {
			self.step_ppu(lcd);
		}

		let start = self.cycle;

		CPU::step(self);

		self.update_timer(self.cycle - start);

		while self.cycle * 16 >= self.ppu_state.cycle {
			self.step_ppu(lcd);
		}
	}

	pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), CartridgeParseError> {
		let cartridge = CartridgeState::from_raw_rom(rom.to_owned())?;
		self.cartridge_state = Some(cartridge);
		Ok(())
	}
}
