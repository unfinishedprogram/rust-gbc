/// Allows reading and writing to memory using a 16 bit address
pub trait MemoryMapper {
	fn read(&self, addr: u16) -> u8;
	fn write(&mut self, addr: u16, value: u8);
}

/// Similar to `MemoryMapper` but allows specifying a source,
/// This is needed for accurate emulation,
pub trait SourcedMemoryMapper: MemoryMapper {
	fn read_from(&self, addr: u16, source: Source) -> u8;
	fn write_from(&mut self, addr: u16, value: u8, source: Source);
}

/// Defines a source for a given read/write
pub enum Source {
	/// From the CPU
	Cpu,

	/// From the PPI
	Ppu,

	/// No source, useful for debugging
	Raw,
}
