use std::ops::{Index, IndexMut};

pub struct Memory {
	bytes: [u8; 0xFFFF],
}

impl Memory {
	pub fn new() -> Self {
		Self { bytes: [0; 0xFFFF] }
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
