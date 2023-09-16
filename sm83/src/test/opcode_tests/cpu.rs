use super::{memory_mapper::FlatMemory, state::TestState};
use crate::{
	registers::{CPURegister16::*, CPURegister8::*},
	CPUState, SM83,
};

#[derive(Default)]
pub struct MockCpu {
	pub memory: FlatMemory,
	pub cpu_state: CPUState,
}

impl SM83<FlatMemory> for MockCpu {
	fn cpu_state(&self) -> &CPUState {
		&self.cpu_state
	}

	fn cpu_state_mut(&mut self) -> &mut CPUState {
		&mut self.cpu_state
	}

	fn memory_mapper(&self) -> &FlatMemory {
		&self.memory
	}

	fn memory_mapper_mut(&mut self) -> &mut FlatMemory {
		&mut self.memory
	}
}

impl From<TestState> for MockCpu {
	fn from(state: TestState) -> Self {
		let mut res = Self::default();
		res.cpu_state.interrupt_master_enable = state.ime == 1;
		res.cpu_state.ie_next = state.ime == 1;

		res.cpu_state.registers[PC] = state.pc;
		res.cpu_state.registers[SP] = state.sp;

		res.cpu_state.registers[A] = state.a;
		res.cpu_state.registers[B] = state.b;
		res.cpu_state.registers[C] = state.c;
		res.cpu_state.registers[D] = state.d;

		res.cpu_state.registers[E] = state.e;
		res.cpu_state.registers[F] = state.f;
		res.cpu_state.registers[H] = state.h;
		res.cpu_state.registers[L] = state.l;

		res.memory.data = state.ram;
		res
	}
}

impl From<MockCpu> for TestState {
	fn from(state: MockCpu) -> Self {
		let mut ram = state.memory.data.clone();
		ram.sort_by(|a, b| a.0.cmp(&b.0));

		TestState {
			pc: state.cpu_state.registers[PC],
			sp: state.cpu_state.registers[SP],
			a: state.cpu_state.registers[A],
			b: state.cpu_state.registers[B],
			c: state.cpu_state.registers[C],
			d: state.cpu_state.registers[D],
			e: state.cpu_state.registers[E],
			f: state.cpu_state.registers[F],
			h: state.cpu_state.registers[H],
			l: state.cpu_state.registers[L],
			ime: if state.cpu_state.interrupt_master_enable {
				1
			} else {
				0
			},
			ram,
		}
	}
}
