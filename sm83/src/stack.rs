use crate::{
	registers::{Addressable, CPURegister16},
	SM83,
};

pub trait CPUStack: SM83 {
	fn push(&mut self, value: u16);
	fn pop(&mut self) -> u16;

	fn push_u8(&mut self, value: u8) {
		let next_sp = self.cpu_state().read(CPURegister16::SP).wrapping_sub(1);
		self.cpu_state_mut().write(CPURegister16::SP, next_sp);
		self.write_8(crate::values::ValueRefU8::Mem(next_sp.into()), value);
	}

	fn pop_u8(&mut self) -> u8 {
		let next_sp = self.cpu_state().read(CPURegister16::SP).wrapping_add(1);
		self.cpu_state_mut().write(CPURegister16::SP, next_sp);
		self.read_8(crate::values::ValueRefU8::Mem(
			next_sp.wrapping_sub(1).into(),
		))
	}
}

impl<M: SM83> CPUStack for M {
	fn push(&mut self, value: u16) {
		self.push_u8((value >> 8) as u8);
		self.push_u8(value as u8);
	}

	fn pop(&mut self) -> u16 {
		let low = self.pop_u8() as u16;
		let high = self.pop_u8() as u16;

		(high << 8) | low
	}
}
