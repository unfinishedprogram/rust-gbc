#![feature(generic_associated_types)]
#![feature(mixed_integer_ops)]

pub mod app;
pub mod components;
pub mod memory;

mod emulator;
use cpu::Cpu;
mod cartridge;
mod cpu;
mod ppu;
mod util;

pub use app::EmulatorManager;
