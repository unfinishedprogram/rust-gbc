use super::cpu::CPUState;

pub struct EmulatorState {
	t_states: u64,
	cpu_state: CPUState,
	v_ram: [[u8; 0x2000]; 2],
	w_ram: [[u8; 0x1000]; 8],
	oam: [u8; 0xA0],
	interupt_register: u8,
}

impl Default for EmulatorState {
	fn default() -> Self {
		Self {
			t_states: 0,
			cpu_state: CPUState::default(),
			v_ram: [[u8; 0x2000]; 2],
			w_ram: [[u8; 0x1000]; 8],
			oam: [u8; 0xA0],
			interupt_register: 0,
		}
	}
}
