use std::{
	cell::RefCell,
	ops::{Index, IndexMut},
};

pub struct Memory {
	bytes: [u8; 0x10000],
	pub t_state: RefCell<u32>,
}

impl Memory {
	pub fn new() -> Self {
		Self {
			bytes: [0; 0x10000],
			t_state: RefCell::new(0),
		}
	}

	pub fn read(&self, addr: u16) -> u8 {
		*self.t_state.borrow_mut() += 1;
		return self.bytes[addr as usize];
	}

	pub fn write(&mut self, addr: u16, value: u8) {
		*self.t_state.borrow_mut() += 1;
		self.bytes[addr as usize] = value;
	}
}

impl Index<u16> for Memory {
	type Output = u8;
	fn index(&self, addr: u16) -> &Self::Output {
		&self.bytes[addr as usize]
	}
}

impl IndexMut<u16> for Memory {
	fn index_mut(&mut self, addr: u16) -> &mut Self::Output {
		&mut self.bytes[addr as usize]
	}
}
