use crate::{app::components::logger, emulator::state::EmulatorState};

pub trait MemoryMapper {
	fn read(&self, addr: u16) -> u8;
	fn write(&mut self, addr: u16, value: u8);
}
