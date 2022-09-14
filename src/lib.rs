#![feature(generic_associated_types)]
#![feature(mixed_integer_ops)]

use cpu::Cpu;
mod cpu;
mod cartridge;
pub mod app;
pub mod components;
pub use app::EmulatorManager;

pub fn load_rom_and_run(processor: &mut Cpu, rom:&[u8], boot_rom:&[u8]) {
    processor.load_cartridge(rom);
    processor.load_boot_rom(boot_rom);
    let start:u16 = 0x0000;
    processor.write_16(cpu::registers::CPURegister16::PC.into(), start.into());
}
