#![feature(generic_associated_types)]

use wasm_bindgen::prelude::*;

mod cpu;

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
    let processor = cpu::CPU::new();
    let inst = 

    // let opcode = cpu::Opcode::from(*processor.read_mem());

    // let instruction = instruction::get_instruction(cpu);
    alert("Done");
}