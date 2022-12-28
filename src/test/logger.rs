use crate::emulator::{
	cpu::{instruction::execute::execute_instruction, registers::CPURegister8, CPU},
	memory_mapper::MemoryMapper,
	EmulatorState,
};

// 0100 nop                 A:01 F:b0 B:00 C:13 D:00 E:d8 H:01 L:4d LY:00 SP:fffe

pub fn log_execute(state: &mut EmulatorState) -> String {
	use CPURegister8::*;
	let pc = state.cpu_state.registers.pc;
	let rs = format!(
		"A:{:02x} F:{:02x} B:{:02x} C:{:02x} D:{:02x} E:{:02x} H:{:02x} L:{:02x} LY:{:02x} SP:{:02x}  Cy:{}",
		state.cpu_state.registers[A],
		state.cpu_state.registers[F],
		state.cpu_state.registers[B],
		state.cpu_state.registers[C],
		state.cpu_state.registers[D],
		state.cpu_state.registers[E],
		state.cpu_state.registers[H],
		state.cpu_state.registers[L],
		state.read(0xFF44),
		state.cpu_state.registers.sp,
		state.get_cycle()*2
	);

	let instruction = state.fetch_next_instruction();
	let inst = format!("{instruction:?}",);

	execute_instruction(instruction, state);
	format!("{pc:04X} {inst:<19} {rs}")
}

pub fn execute(state: &mut EmulatorState) {
	state.step();
}
