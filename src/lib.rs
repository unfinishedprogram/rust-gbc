use wasm_bindgen::prelude::*;

mod emulator;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn load_rom_and_run(rom:js_sys::Uint8Array) {
    let mut state = emulator::EmulatorState::new();
    unsafe {
        // Load raw rom into memory
        rom.raw_copy_to_ptr(state.memory.as_mut_ptr());
    }
    alert("Done");
}