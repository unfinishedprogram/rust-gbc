use super::{values::ValueRefU16, Gameboy, CPU};

pub trait GBStack {
	fn push(&mut self, value: u16);
	fn pop(&mut self) -> u16;
}

impl GBStack for Gameboy {
	fn push(&mut self, value: u16) {
		self.cpu_state.registers.sp = self.cpu_state.registers.sp.wrapping_sub(2);
		self.write_16(&ValueRefU16::Mem(self.cpu_state.registers.sp), value)
	}

	fn pop(&mut self) -> u16 {
		self.cpu_state.registers.sp = self.cpu_state.registers.sp.wrapping_add(2);
		self.read_16(&ValueRefU16::Mem(
			self.cpu_state.registers.sp.wrapping_sub(2),
		))
	}
}
