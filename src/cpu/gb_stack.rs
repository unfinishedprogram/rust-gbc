use super::{Cpu, values::ValueRefU16, registers::CPURegister16};

pub trait GBStack {
	fn push(&mut self, value: u16);
	fn pop(&mut self) -> u16;
}

impl GBStack for Cpu {
	fn push(&mut self, value: u16) {
		let sp = self.read_16(CPURegister16::SP.into());
		self.write_16(ValueRefU16::Mem(sp), value);
		self.write_16(CPURegister16::SP.into(), sp - 2);
	}

	fn pop(&mut self) -> u16 {
		let sp = self.read_16(CPURegister16::SP.into());
		self.write_16(CPURegister16::SP.into(), sp + 2);
		self.read_16(ValueRefU16::Mem(sp))
	}
}

#[cfg(test)]
mod tests {
  use crate::cpu::{Cpu, values::ValueRefU16, registers::CPURegister16};
	use super::GBStack;

	#[test]
	fn stack_tests() {
		let mut cpu = Cpu::new();
		cpu.write_16(ValueRefU16::Reg(CPURegister16::SP), 0xE000);
		cpu.push(255);
		assert_eq!(255, cpu.pop());
		cpu.push(0);
		cpu.push(1);
		cpu.push(4);
		assert_eq!(4, cpu.pop());
		assert_eq!(1, cpu.pop());
		assert_eq!(0, cpu.pop());
	}
}