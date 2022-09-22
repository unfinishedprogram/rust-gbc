#![feature(slice_flatten)]

pub mod app;
pub mod components;
pub mod lcd;
pub mod memory;

mod cartridge;
mod cpu;
mod emulator;
mod ppu;
mod util;

pub use app::EmulatorManager;
