use std::fs;

use crate::emulator::{
	cpu::{
		instruction::execute::execute_instruction, registers::CPURegister8, values::ValueRefU8, CPU,
	},
	EmulatorState,
};

// 0100 nop                 A:01 F:b0 B:00 C:13 D:00 E:d8 H:01 L:4d LY:00 SP:fffe

pub fn log_execute(state: &mut EmulatorState) -> String {
	use CPURegister8::*;
	use ValueRefU8::Reg;
	let pc = state.cpu_state.registers.pc;
	let rs = format!(
		"A:{:02x} F:{:02x} B:{:02x} C:{:02x} D:{:02x} E:{:02x} H:{:02x} L:{:02x} SP:{:02x}",
		state.read_8(Reg(A)),
		state.read_8(Reg(F)),
		state.read_8(Reg(B)),
		state.read_8(Reg(C)),
		state.read_8(Reg(D)),
		state.read_8(Reg(E)),
		state.read_8(Reg(H)),
		state.read_8(Reg(L)),
		state.cpu_state.registers.sp
	);

	let instruction = state.fetch_next_instruction();
	let inst = format!("{:?}", instruction);
	execute_instruction(instruction, state);
	return format!("{pc:04X} {inst:<19} {rs}  ");
}
