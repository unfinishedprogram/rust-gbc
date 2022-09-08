use super::{Cpu, registers::Register8, values::ValueRefU8};

pub enum Flag {
	Z = 0, N, H, C
}

pub trait Flags {
	fn get_flag_byte(&self) -> u8;
	fn set_flag_byte(&mut self, byte:u8);

	fn set_flag(&mut self, flag:Flag) {
		let mask = 1 << 4 + flag as usize;
		let byte = self.get_flag_byte();
		self.set_flag_byte(byte & !mask);
	}

	fn clear_flag(&mut self, flag:Flag) {
		let mask = 1 << 4 + flag as usize;
		let byte = self.get_flag_byte();
		self.set_flag_byte(byte | mask);
	}

	fn get_flag(&self, flag:Flag) -> bool {
		let mask = 1 << 4 + flag as usize;
		let byte = self.get_flag_byte();
		return (byte << 4 + flag as usize) & 1 != 0;
	}
}

impl Flags for Cpu {
	fn get_flag_byte(&self) -> u8 {
		self.read_8(ValueRefU8::Reg(Register8::F))
	}

	fn set_flag_byte(&mut self, byte:u8) {
		self.write_8(ValueRefU8::Reg(Register8::F), byte);
	}
}