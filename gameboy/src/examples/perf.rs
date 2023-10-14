use std::fs::File;

use gameboy::Gameboy;
use pprof::{flamegraph::Options, ProfilerGuardBuilder};

pub fn main() {
	let runs = 10;
	let cycles_per_run = 1_048_576 * 32; // ~32 seconds worth

	let guard = ProfilerGuardBuilder::default()
		.frequency(1000)
		.build()
		.unwrap();

	let mut gb = Gameboy::cgb();
	gb.load_rom(
		include_bytes!("../../../test_data/blargg/cpu_instrs/cpu_instrs.gb"),
		None,
	);

	for run in 0..runs {
		let mut gb = gb.clone();
		println!("run: #{}", run + 1);

		for _ in 0..cycles_per_run {
			gb.step();
		}
	}

	match guard.report().build() {
		Ok(report) => {
			let output_path = "flamegraph.svg";
			let file = File::create(output_path).unwrap();
			let mut options = Options::default();
			report.flamegraph_with_options(file, &mut options).unwrap();
			println!("flamegraph saved to {}", output_path)
		}
		Err(e) => println!("error: {}", e),
	}
}
