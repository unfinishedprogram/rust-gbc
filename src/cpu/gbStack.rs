use super::{Cpu, values::ValueRefU16, registers::Register16};

pub trait GBStack {
	fn push(&mut self, value: u16);
	fn pop(&mut self) -> u16;
}

impl GBStack for Cpu {
	fn push(&mut self, value: u16) {
		self.write_16(
			ValueRefU16::Mem(
				self.read_16(ValueRefU16::Reg(Register16::SP))
			),
			value,
		);

		self.write_16(
			ValueRefU16::Reg(Register16::SP), 
			self.read_16(ValueRefU16::Reg(Register16::SP))-2
		);
	}

	fn pop(&mut self) -> u16 {
		let res = self.read_16(
			ValueRefU16::Mem(
				self.read_16(ValueRefU16::Reg(Register16::SP))
			)
		);
		
		self.write_16(
			ValueRefU16::Reg(Register16::SP), 
			self.read_16(ValueRefU16::Reg(Register16::SP))+2
		);
		
		return res;
	}
}