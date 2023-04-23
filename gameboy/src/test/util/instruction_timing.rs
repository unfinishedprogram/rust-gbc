use sm83::{Flags, SM83};

use crate::test::boot::cgb_test_instance;

/// Instrs will be loaded as if the first instructions in the rom
pub fn expect_instr_timing(name: &str, instrs: &[u8], steps: usize, expected: u64, flag: u8) {
	let taken = get_cycles_taken(instrs, steps, flag);

	if taken != expected {
		panic!("Failed on {name}. Took: {taken}, Expected: {expected}");
	}
}

/// Computes the number of cycles taken
/// Flags are used to force conditions
pub fn get_cycles_taken(instrs: &[u8], steps: usize, flag: u8) -> u64 {
	let mut state = cgb_test_instance();
	let start_cycle = state.get_cycle();

	// Clear all the flags
	state.cpu_state_mut().clear_flag(0xFF);
	state.cpu_state_mut().set_flag(flag);

	if let Some(cart) = &mut state.cartridge_state {
		for (i, instr) in instrs.iter().enumerate() {
			cart.0.rom_banks[0][0x100 + i] = *instr;
		}
	}

	for _ in 0..steps {
		state.step();
	}

	state.get_cycle() - start_cycle
}
