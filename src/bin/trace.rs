use gameboy::sm83::memory_mapper::MemoryMapper;
use gameboy::sm83::registers::{Addressable, CPURegister16, CPURegister8};
use gameboy::Gameboy;

// 0100 nop                 A:01 F:b0 B:00 C:13 D:00 E:d8 H:01 L:4d LY:00 SP:fffe

pub fn main() {
	let mut gb = Gameboy::cgb();
	gb.load_rom(include_bytes!("../../roms/games/Wario Land 3.gbc"), None);
	gb.run_until_boot();
	use CPURegister8::*;

	gb.cpu_state.write(A, 0x11);
	gb.cpu_state.write(F, 0xb0);
	gb.cpu_state.write(B, 0x00);
	gb.cpu_state.write(C, 0x13);
	gb.cpu_state.write(D, 0x00);
	gb.cpu_state.write(E, 0xd8);
	gb.cpu_state.write(H, 0x01);
	gb.cpu_state.write(L, 0x4d);

	gb.t_states = 0;
	for _ in 0..100 {
		let cycle = gb.t_states / 2;

		let sp = gb.cpu_state.read(CPURegister16::SP);
		let pc = gb.cpu_state.read(CPURegister16::PC);
		let rs = format!(
                "A:{:02x} F:{:02x} B:{:02x} C:{:02x} D:{:02x} E:{:02x} H:{:02x} L:{:02x} LY:{:02x} SP:{:02x}  Cy:{}",
                gb.cpu_state.read(A),
                gb.cpu_state.read(F),
                gb.cpu_state.read(B),
                gb.cpu_state.read(C),
                gb.cpu_state.read(D),
                gb.cpu_state.read(E),
                gb.cpu_state.read(H),
                gb.cpu_state.read(L),
                gb.read(0xFF44),
                sp,
                cycle,
            );

		let instruction = gb.step();
		if let Some(instruction) = instruction {
			let inst = format!("{instruction:?}");
			println!("{pc:04X} {inst:<19} {rs}");
		}
	}
}
