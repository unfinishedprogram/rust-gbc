use std::{
	fs::File,
	io::{BufRead, BufReader, Read},
};

use crate::{
	emulator::{
		cpu::{instruction::execute::execute_instruction, registers::CPURegister8, CPU},
		lcd::LCDDisplay,
		memory_mapper::MemoryMapper,
		ppu::PPU,
		timer::Timer,
		EmulatorState,
	},
	test::mocks::mock_lcd::MockLCD,
};

fn debug_step_state(state: &mut EmulatorState, lcd: &mut dyn LCDDisplay) -> Option<String> {
	while state.cycle * 16 > state.ppu_state.cycle {
		state.step_ppu(lcd);
	}

	let start = state.cycle;

	if state.halted {
		if state.interrupt_pending() {
			state.halted = false;
		} else {
			state.cycle += 1;
		}
	} else {
		use CPURegister8::*;
		let pc = state.cpu_state.registers.pc;
		let rs = format!(
            "A:{:02x} F:{:02x} B:{:02x} C:{:02x} D:{:02x} E:{:02x} H:{:02x} L:{:02x} SP:{:02x} LY:{:02x} Cy:{}",
            state.cpu_state.registers[A],
            state.cpu_state.registers[F],
            state.cpu_state.registers[B],
            state.cpu_state.registers[C],
            state.cpu_state.registers[D],
            state.cpu_state.registers[E],
            state.cpu_state.registers[H],
            state.cpu_state.registers[L],
            state.cpu_state.registers.sp,
            state.read(0xFF44),
            state.cycle*2 + 2940552
	    );

		let instruction = state.get_next_instruction_or_interrupt();
		let inst = format!("{instruction:?}");
		execute_instruction(instruction, state);

		state.update_timer(state.cycle - start);

		while state.cycle * 16 >= state.ppu_state.cycle {
			state.step_ppu(lcd);
		}
		return Some(format!("{pc:04X} {inst:<19} {rs}"));
	}

	state.update_timer(state.cycle - start);

	while state.cycle * 16 >= state.ppu_state.cycle {
		state.step_ppu(lcd);
	}

	None
}

#[test]
fn link() {
	let rom_name = "games/LegendOfZelda";
	let mut lcd = MockLCD::default();
	let mut state = EmulatorState::default();

	let rom_handle = File::open(format!("roms/{rom_name}.gb"))
		.unwrap_or_else(|_| panic!("roms/{rom_name}.gb not found"));

	let rom: Vec<u8> = BufReader::new(rom_handle)
		.bytes()
		.map(|byte| byte.unwrap())
		.collect();

	state.load_rom(&rom).unwrap();

	let handle = File::open("logs/LegendOfZelda.log").unwrap();

	let mut lines = BufReader::new(handle).lines();
	let mut lc = 0;
	let mut last_line: String = "".to_owned();
	for _ in 0..10000000 {
		if let Some(res) = debug_step_state(&mut state, &mut lcd) {
			let line = lines.next().unwrap().unwrap();
			lc += 1;
			if res != line {
				panic!(
					"
					Line: {lc}
					Before: {last_line}
					Result: {res}
					Expect: {line}
					"
				)
			}
			last_line = line;
		}
	}
}
