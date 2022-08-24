use std::ops::Index;
use super::values::{ U16Value };

pub enum Register8 {
	A = 0, 
	B, C, 
	D, E, 
	F, H, L,
}
pub struct Registers<'a> {
	bytes:[u8;12],
	
	sp:CompositeU16<'a>, 
	pc:CompositeU16<'a>,
	af:CompositeU16<'a>,				
	bc:CompositeU16<'a>,
	de:CompositeU16<'a>,
	hl:CompositeU16<'a>,
}

pub struct CompositeU16<'a>(pub &'a u8, pub &'a u8);

impl <'a>U16Value<'_> for CompositeU16<'a> {
	fn get(&self) -> u16 {
		(*self.0 as u16) << 8 | *self.1 as u16
	}

	fn set(&mut self, value:u16) {
		*self.0 = ((value & 0xFF00) >> 8) as u8;
		*self.1 = (value & 0xFF) as u8;
	}
}

impl <'a>Index<Register16> for Registers<'a> {
	type Output = dyn U16Value<'a>;

	fn index(&self, reg:Register16) -> &Self::Output {
		match reg {
			Register16::AF => &self.af,
			Register16::BC => &self.bc,
			Register16::DE => &self.de,
			Register16::HL => &self.hl,
			Register16::SP => &self.sp,
			Register16::PC => &self.pc,
		}
	}
}

impl <'a>Index<Register8> for Registers<'a> {
	type Output = u8;

	fn index(&self, reg:Register8) -> &Self::Output {
		match reg {
			Register8::A => &self.bytes[0],
			Register8::B => &self.bytes[1],
			Register8::C => &self.bytes[2],
			Register8::D => &self.bytes[3],
			Register8::E => &self.bytes[4],
			Register8::F => &self.bytes[5],
			Register8::H => &self.bytes[6],
			Register8::L => &self.bytes[7],
		}
	}
}

pub enum Register16 {
	AF, BC,
	DE, HL, 
	SP, PC, 
}

impl<'a> Registers<'a> {
	pub fn new() -> Registers<'a> {
		//								  [A  B  C  D  E  F  H  L  S  P  P  C]
		let bytes:[u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

		Registers {
			bytes,
			af: CompositeU16(&bytes[0], &bytes[5]), // AF				
			bc: CompositeU16(&bytes[1], &bytes[2]), // BC
			de: CompositeU16(&bytes[3], &bytes[4]), // DE
			hl: CompositeU16(&bytes[6], &bytes[7]), // HL
			sp: CompositeU16(&bytes[8], &bytes[9]), 
			pc: CompositeU16(&bytes[10], &bytes[11]),
		}
	}

	pub fn get_u8(&self, reg:Register8) -> u8 {
		return self[reg];
	}

	pub fn set_u8(&mut self, reg:Register8, value:u8) {
		self[reg] = value;
	}

	pub fn get_u16(&self, reg:Register16) -> u16 {
		return self[reg].get();
	}

	pub fn set_u16(&mut self, reg:Register16, value:u16) {
		return self[reg].set(value);
	}
}