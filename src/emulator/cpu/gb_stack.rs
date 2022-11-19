use crate::emulator::state::EmulatorState;

use super::{registers::CPURegister16, values::ValueRefU16, CPU};

pub trait GBStack {
	fn push(&mut self, value: u16);
	fn pop(&mut self) -> u16;
}

impl GBStack for EmulatorState {
	fn push(&mut self, value: u16) {
		let sp = self.read_16(CPURegister16::SP.into());
		self.write_16(CPURegister16::SP.into(), sp - 2);
		self.write_16(ValueRefU16::Mem(sp - 2), value);
	}

	fn pop(&mut self) -> u16 {
		let sp = self.read_16(CPURegister16::SP.into());
		self.write_16(CPURegister16::SP.into(), sp + 2);
		self.read_16(ValueRefU16::Mem(sp))
	}
}
