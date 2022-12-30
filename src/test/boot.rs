use crate::emulator::EmulatorState;
extern crate test;

use test::Bencher;

#[bench]
pub fn bench_boot(b: &mut Bencher) {
	let mut state = EmulatorState::default();

	b.iter(|| {
		state.step();
		// state.run_until_boot();
	})
}
