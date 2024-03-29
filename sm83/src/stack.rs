use crate::{
	registers::{Addressable, CPURegister16},
	values::ValueRefU16,
	SM83,
};

pub trait CPUStack: SM83 {
	fn push(&mut self, value: u16);
	fn pop(&mut self) -> u16;
}

impl<M: SM83> CPUStack for M {
	fn push(&mut self, value: u16) {
		let next_sp = self.cpu_state().read(CPURegister16::SP).wrapping_sub(2);

		self.cpu_state_mut().write(CPURegister16::SP, next_sp);

		self.write_16(
			ValueRefU16::Mem(self.cpu_state().read(CPURegister16::SP)),
			value,
		);
	}

	fn pop(&mut self) -> u16 {
		let next_sp = self.cpu_state().read(CPURegister16::SP).wrapping_add(2);

		self.cpu_state_mut().write(CPURegister16::SP, next_sp);

		self.read_16(ValueRefU16::Mem(
			self.cpu_state().read(CPURegister16::SP).wrapping_sub(2),
		))
	}
}
