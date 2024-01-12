// Manages audio buffers and synchronization
pub struct Audio {
	// Number of samples per buffer per channel
	buffer_size: usize,
	// Number of samples per second
	sample_rate: usize,

	// Size is double buffer size because we have two channels
	buffer: Vec<f32>,

	current_t_states: usize,

	t_states_per_buffer: f64,
}

impl Default for Audio {
	fn default() -> Self {
		Self {
			buffer_size: 1024,
			sample_rate: 44100,
		}
	}
}

impl Audio {
	pub fn step(&mut self, apu: &APU, t_states: usize) {
		let (left, right) = apu.sample();
	}

	pub fn new(buffer_size: usize, sample_rate: usize) -> Self {
		Self {
			buffer_size,
			sample_rate,
		}
	}
}
