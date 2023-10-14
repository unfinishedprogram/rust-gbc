use std::time::Instant;

use gameboy::Gameboy;

pub fn main() {
	let runs = 10;
	let cycles_per_run = 1_048_576 * 32; // ~32 seconds worth

	let mut gb = Gameboy::cgb();
	gb.load_rom(
		include_bytes!("../../../test_data/blargg/cpu_instrs/cpu_instrs.gb"),
		None,
	);

	for run in 0..runs {
		let mut gb = gb.clone();
		println!("run: #{}", run + 1);
		let start = Instant::now();

		for _ in 0..cycles_per_run {
			gb.step();
		}

		let elapsed = Instant::now() - start;
		let frame_count = gb.ppu.frame;
		let ms_per_frame = elapsed.as_millis() as f64 / frame_count as f64;
		println!("Elapsed: {:?}", elapsed);
		println!("Frame Count: {:?}", frame_count);
		println!("MS/Frame: {:?}", ms_per_frame);
	}
}
