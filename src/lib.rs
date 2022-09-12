#![feature(generic_associated_types)]
#![feature(mixed_integer_ops)]

use wasm_bindgen::prelude::*;
mod cpu;
mod cartridge;

#[macro_use]
mod logger;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = window, js_name = log)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn load_rom_and_run(rom:&[u8], boot_rom:&[u8]) -> cartridge::header::Header {
    let cart = cartridge::header::Header::from(rom.clone());
    console_log!("{:#?}", cart);

    let mut processor = cpu::Cpu::new();
    processor.load_cartridge(rom);
    processor.load_boot_rom(boot_rom);
    let start:u16 = 0x0000;
    processor.write_16(cpu::registers::CPURegister16::PC.into(), start.into());

    for _ in 0..1000 {
        console_log!("{}", processor.read_16(cpu::registers::CPURegister16::PC.into()));
        console_log!("{:?}", processor.execute_next_instruction());
    }

    return cart;
}