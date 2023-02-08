use sm83::memory_mapper::{MemoryMapper, Source, SourcedMemoryMapper};

#[derive(Default)]
pub struct FlatMemory {
	pub data: Vec<(u16, u8)>,
}

impl MemoryMapper for FlatMemory {
	fn read(&self, addr: u16) -> u8 {
		for (index, val) in &self.data {
			if *index == addr {
				return *val;
			}
		}
		0
	}

	fn write(&mut self, addr: u16, value: u8) {
		for (index, val) in &mut self.data {
			if *index == addr {
				*val = value;
				return;
			}
		}
		self.data.push((addr, value))
	}
}

impl SourcedMemoryMapper for FlatMemory {
	fn read_from(&self, addr: u16, _source: Source) -> u8 {
		self.read(addr)
	}

	fn write_from(&mut self, addr: u16, value: u8, _source: Source) {
		self.write(addr, value)
	}
}
