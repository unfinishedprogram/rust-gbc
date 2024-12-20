use serde::{Deserialize, Serialize};

use crate::apu::Apu;
use std::collections::VecDeque;

// Manages audio buffers and synchronization
#[derive(Clone, Serialize, Deserialize)]
pub struct Audio {
	raw_samples: VecDeque<(f32, f32)>,
	t_states: usize,
	current_sample: usize,
	last_pull_sample: usize,
	sample_countdown: usize,
}

impl Default for Audio {
	fn default() -> Self {
		Audio {
			raw_samples: VecDeque::new(),
			t_states: 0,
			current_sample: 0,
			last_pull_sample: 0,
			sample_countdown: 4,
		}
	}
}

impl Audio {
	pub fn step(&mut self, apu: &mut Apu, t_states: usize) {
		for _ in 0..t_states {
			self.step_single(apu);
		}
	}

	fn step_single(&mut self, apu: &mut Apu) {
		self.sample_countdown -= 1;
		if self.sample_countdown == 0 {
			self.sample_countdown = 4;
			let (left, right) = apu.sample();
			self.raw_samples.push_back((left, right));
			if self.raw_samples.len() > 80_000 {
				self.raw_samples.pop_front();
			}
		}
	}

	// This is expected to happen once per frame
	pub fn pull_samples(&mut self, samples: usize) -> Vec<(f32, f32)> {
		let mut res = Vec::with_capacity(samples);

		let raw_samples = self.raw_samples.len();
		let ratio = raw_samples as f64 / samples as f64;

		// Resample buffer to the requested size
		for i in 0..samples {
			let raw_index = (i as f64 * ratio).floor() as usize;
			if raw_index >= raw_samples {
				break;
			}
			res.push(self.raw_samples[raw_index]);
		}

		self.raw_samples.clear();
		res
	}

	pub fn buffered_samples(&self) -> usize {
		self.raw_samples.len()
	}
}
