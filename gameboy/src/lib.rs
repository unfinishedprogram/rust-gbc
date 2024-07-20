#![feature(assert_matches)]
#![feature(test)]
#![feature(async_closure)]

mod apu;
pub mod audio;
pub mod cartridge;
pub mod cgb;
mod dma_controller;
pub mod io_registers;
pub mod joypad;
pub mod lcd;
pub mod memory_mapper;
mod oam_dma;
pub mod ppu;
pub mod save_state;
mod state;
mod timer;
mod util;
pub mod work_ram;
pub use state::{Gameboy, Mode};

// Re-export the sm83 crate for debugging
pub use sm83;

#[cfg(test)]
pub mod test;
