#![feature(exclusive_range_pattern)]
#![feature(assert_matches)]
#![feature(test)]
#![feature(local_key_cell_methods)]
#![feature(async_closure)]

pub mod cartridge;
pub mod cgb;
pub mod controller;
mod dma_controller;
mod flags;
pub mod io_registers;
pub mod lcd;
pub mod memory_mapper;
mod oam_dma;
pub mod ppu;
pub mod save_state;
mod state;
mod timer;
mod util;
mod work_ram;
pub use state::{Gameboy, Mode};

#[cfg(test)]
pub mod test;
