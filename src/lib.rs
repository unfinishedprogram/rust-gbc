#![feature(generic_associated_types)]
#![feature(mixed_integer_ops)]

use wasm_bindgen::prelude::*;

mod cpu;
mod cartridge;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = window, js_name = log)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn load_rom_and_run(rom:&[u8]) -> cartridge::header::Header {
    let cart = cartridge::header::Header::from(rom.clone());
    console_log!("cart{:#?}", cart);
    return cart;

    // let mut processor = cpu::Cpu::new();
    // processor.execute_next_instruction();
}