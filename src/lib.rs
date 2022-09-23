#![feature(slice_flatten)]

pub mod app;
pub mod cartridge;
pub mod components;
pub mod cpu;
pub mod emulator;
// pub mod flags;
pub mod lcd;
pub mod memory;
pub mod memory_registers;
pub mod ppu;
pub mod util;
pub use app::EmulatorManager;
