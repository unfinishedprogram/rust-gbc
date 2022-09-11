#![feature(generic_associated_types)]
#![feature(mixed_integer_ops)]

use wasm_bindgen::prelude::*;

mod cpu;
mod cartridge;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn load_rom_and_run(rom:js_sys::Uint8Array) {
    let _ = rom;
    // let mut state = emulator::EmulatorState::new();
    // unsafe {
        // Load raw rom into memory
        // rom.raw_copy_to_ptr(state.memory.as_mut_ptr());
    // }
    let mut processor = cpu::Cpu::new();
    processor.execute_next_instruction();

    // let opcode = cpu::Opcode::from(*processor.read_mem());

    // let instruction = instruction::get_instruction(cpu);
    alert("Done");
}