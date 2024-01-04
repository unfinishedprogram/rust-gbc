use super::{memory_mapper::FlatMemory, state::TestState};
use crate::{
	memory_mapper::{MemoryMapper, SourcedMemoryMapper},
	registers::{Addressable, CPURegister16::*, CPURegister8::*},
	CPUState, SM83,
};

#[derive(Default)]
pub struct MockCpu {
	pub memory: FlatMemory,
	pub cpu_state: CPUState,
}

impl MemoryMapper for MockCpu {
	fn read(&self, addr: u16) -> u8 {
		self.memory.read(addr)
	}

	fn write(&mut self, addr: u16, value: u8) {
		self.memory.write(addr, value)
	}
}

impl SourcedMemoryMapper for MockCpu {
	fn read_from(&self, addr: u16, source: crate::memory_mapper::Source) -> u8 {
		self.memory.read_from(addr, source)
	}

	fn write_from(&mut self, addr: u16, value: u8, source: crate::memory_mapper::Source) {
		self.memory.write_from(addr, value, source)
	}
}

impl SM83 for MockCpu {
	fn cpu_state(&self) -> &CPUState {
		&self.cpu_state
	}

	fn cpu_state_mut(&mut self) -> &mut CPUState {
		&mut self.cpu_state
	}
}

impl From<TestState> for MockCpu {
	fn from(state: TestState) -> Self {
		let mut res = Self::default();
		res.cpu_state.enable_interrupts();

		if state.ime == 1 {
			res.cpu_state.enable_interrupts();
			res.cpu_state.tick_ie_delay();
		}

		res.cpu_state.write(PC, state.pc);
		res.cpu_state.write(SP, state.sp);
		res.cpu_state.write(A, state.a);
		res.cpu_state.write(B, state.b);
		res.cpu_state.write(C, state.c);
		res.cpu_state.write(D, state.d);
		res.cpu_state.write(E, state.e);
		res.cpu_state.write(F, state.f);
		res.cpu_state.write(H, state.h);
		res.cpu_state.write(L, state.l);

		res.memory.data = state.ram;
		res
	}
}

impl From<MockCpu> for TestState {
	fn from(state: MockCpu) -> Self {
		let mut ram = state.memory.data.clone();
		ram.sort_by(|a, b| a.0.cmp(&b.0));

		TestState {
			pc: state.cpu_state.read(PC),
			sp: state.cpu_state.read(SP),
			a: state.cpu_state.read(A),
			b: state.cpu_state.read(B),
			c: state.cpu_state.read(C),
			d: state.cpu_state.read(D),
			e: state.cpu_state.read(E),
			f: state.cpu_state.read(F),
			h: state.cpu_state.read(H),
			l: state.cpu_state.read(L),
			ime: if state.cpu_state.ime() { 1 } else { 0 },
			ram,
		}
	}
}
