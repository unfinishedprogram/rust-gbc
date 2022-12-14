#![feature(exclusive_range_pattern)]
#![feature(assert_matches)]
#![feature(test)]
#![feature(local_key_cell_methods)]
#![feature(async_closure)]

pub mod cartridge;
pub mod cgb;
pub mod controller;
pub mod cpu;
pub mod flags;
pub mod io_registers;
pub mod lcd;
pub mod memory_mapper;
pub mod ppu;
pub mod save_state;
pub mod state;
pub mod timer;
pub mod util;
pub use state::Gameboy;

#[cfg(test)]
pub mod test;
