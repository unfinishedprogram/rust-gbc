use crate::{memory_mapper::SourcedMemoryMapper, values::ValueRefU16, SM83};

pub trait CPUStack<M: SourcedMemoryMapper>: SM83<M> {
	fn push(&mut self, value: u16);
	fn pop(&mut self) -> u16;
}

impl<M: SourcedMemoryMapper, C: SM83<M>> CPUStack<M> for C {
	fn push(&mut self, value: u16) {
		let next_sp = self.cpu_state().registers.sp.wrapping_sub(2);
		self.cpu_state_mut().registers.sp = next_sp;
		self.write_16(ValueRefU16::Mem(self.cpu_state().registers.sp), value);
	}

	fn pop(&mut self) -> u16 {
		let next_sp = self.cpu_state().registers.sp.wrapping_add(2);
		self.cpu_state_mut().registers.sp = next_sp;
		self.read_16(ValueRefU16::Mem(
			self.cpu_state().registers.sp.wrapping_sub(2),
		))
	}
}
