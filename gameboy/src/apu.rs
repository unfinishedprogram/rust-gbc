mod channel;
mod length_counter;
mod lfsr;
mod noise;
mod timer;
mod volume_envelope;

use crate::{
	cgb::Speed,
	sm83::memory_mapper::MemoryMapper,
	util::bits::{falling_edge, BIT_5, BIT_6},
};

use serde::{Deserialize, Serialize};
use timer::Timer;

use self::{channel::Channel, noise::Noise};

// Audio Processing Unit
// https://gbdev.io/pandocs/Audio_details.html#audio-details
#[derive(Clone, Serialize, Deserialize)]
pub struct APU {
	prev_div: u8,
	frame_sequencer: u8,

	// square1: Square,
	// square2: Square,
	// wave: Wave,
	noise: Noise,

	nr50: u8,
	nr51: u8,
}

#[derive(Clone, Serialize, Deserialize)]
struct ChannelRegisters {
	nrx0: u8,
	nrx1: u8,
	nrx2: u8,
	nrx3: u8,
	nrx4: u8,
	timer: Timer,
}

#[derive(Clone, Serialize, Deserialize)]
struct Square {
	nrx0: u8,
	nrx1: u8,
	nrx2: u8,
	nrx3: u8,
	nrx4: u8,
	timer: Timer,
}

#[derive(Clone, Serialize, Deserialize)]
struct Wave {
	nrx0: u8,
	nrx1: u8,
	nrx2: u8,
	nrx3: u8,
	nrx4: u8,
	wave_ram: [u8; 0x10],
	timer: Timer,
}

// There are 4 sound channels each with a generator and a DAC
// Each generator produces values from 0 to 15 or 0x0-0XF
// The DAC then translates this into an "analog" value between -1 and 1

// The four analog channel outputs are then fed into the mixer, which selectively adds them (depending on NR51)
// into two analog outputs (Left and Right). Thus, the analog range of those outputs is 4× that of each channel, -4 to 4.
// Then these final outputs are scaled based on NR50 and output to the speakers.
// NOTE: this scaling can never silence a non-silent input.

// TODO: Implement PCM registers CGB only

impl APU {
	pub fn step_t_state(&mut self, div: u8, speed: Speed) {
		let increment_clock = {
			let div_bit_mask = match speed {
				Speed::Normal => BIT_5,
				Speed::Double => BIT_6,
			};

			let res = falling_edge(self.prev_div, div, div_bit_mask);
			self.prev_div = div;
			res
		};

		if increment_clock {
			self.step_frame_sequencer();
		}
	}

	// Ticked on the falling edge of the div register's 5th bit (6th bit in double speed mode)
	// This should tick at 512hz
	fn step_frame_sequencer(&mut self) {
		self.frame_sequencer = self.frame_sequencer.wrapping_add(1);
		match self.frame_sequencer & 7 {
			0 | 4 => self.tick_length_ctr(),
			2 | 6 => {
				self.tick_length_ctr();
				self.tick_sweep();
			}
			7 => self.tick_vol_env(),
			1 | 3 | 5 => {}
			0x8..=u8::MAX => unreachable!(),
		}
	}

	fn tick_sweep(&self) {}
	fn tick_length_ctr(&self) {}
	fn tick_vol_env(&self) {}

	fn master_volume(&self) -> (f32, f32) {
		let left = self.nr50 & 0b111;
		let right = (self.nr50 >> 4) & 0b111;

		let left = (left + 1) as f32 / 15.0;
		let right = (right + 1) as f32 / 15.0;

		(left, right)
	}

	fn sample_mixer(&mut self) -> (f32, f32) {
		let noise = self.noise.sample();

		(noise, noise)
	}

	pub fn sample(&mut self) -> (f32, f32) {
		let (v_left, v_right) = self.master_volume();
		let (left, right) = self.sample_mixer();

		(left * v_left, right * v_right)
	}
}

impl Default for APU {
	fn default() -> Self {
		Self {
			prev_div: 0,
			frame_sequencer: 0,

			// square1: Square::default(),
			// square2: Square::default(),
			// wave: Wave::default(),
			noise: Noise::new(),

			nr50: 0,
			nr51: 0,
		}
	}
}

impl MemoryMapper for APU {
	fn read(&self, addr: u16) -> u8 {
		// Unused lookup table
		// NRx0 NRx1 NRx2 NRx3 NRx4
		// ---------------------------
		// NR1x  $80  $3F $00  $FF  $BF
		// NR2x  $FF  $3F $00  $FF  $BF
		// NR3x  $7F  $FF $9F  $FF  $BF
		// NR4x  $FF  $FF $00  $00  $BF
		// NR5x  $00  $00 $70

		let unused_mask: u8 = match addr {
			0xFF10 => 0x80,
			0xFF11 => 0x3F,
			0xFF12 => 0x00,
			0xFF13 => 0xFF,
			0xFF14 => 0xBF,

			0xFF15 => 0xFF,
			0xFF16 => 0x3F,
			0xFF17 => 0x00,
			0xFF18 => 0xFF,
			0xFF19 => 0xBF,

			0xFF1A => 0x7F,
			0xFF1B => 0xFF,
			0xFF1C => 0x9F,
			0xFF1D => 0xFF,
			0xFF1E => 0xBF,

			0xFF1F => 0xFF,
			0xFF20 => 0xFF,
			0xFF21 => 0x00,
			0xFF22 => 0x00,
			0xFF23 => 0xBF,

			0xFF24 => 0x00,
			0xFF25 => 0x00,

			_ => unreachable!(),
		};

		let value = match addr {
			// 0xFF10 => self.square1.nrx0,
			// 0xFF11 => self.square1.nrx1,
			// 0xFF12 => self.square1.nrx2,
			// 0xFF13 => self.square1.nrx3,
			// 0xFF14 => self.square1.nrx4,

			// 0xFF15 => self.square2.nrx0, // Unused
			// 0xFF16 => self.square2.nrx1,
			// 0xFF17 => self.square2.nrx2,
			// 0xFF18 => self.square2.nrx3,
			// 0xFF19 => self.square2.nrx4,

			// 0xFF1A => self.wave.nrx0,
			// 0xFF1B => self.wave.nrx1,
			// 0xFF1C => self.wave.nrx2,
			// 0xFF1D => self.wave.nrx3,
			// 0xFF1E => self.wave.nrx4,
			0xFF1F => self.noise.read_nrx0(), // Unused
			0xFF20 => self.noise.read_nrx1(),
			0xFF21 => self.noise.read_nrx2(),
			0xFF22 => self.noise.read_nrx3(),
			0xFF23 => self.noise.read_nrx4(),

			0xFF24 => self.nr50, // NR50
			0xFF25 => self.nr51, // NR51

			// 0xFF30..0xFF40 => self.wave.wave_ram[addr as usize - 0xFF30],
			_ => 0xFF,
			_ => unreachable!(),
		};

		value | unused_mask
	}

	fn write(&mut self, addr: u16, value: u8) {
		match addr {
			// 0xFF10 => self.square1.nrx0 = value,
			// 0xFF11 => self.square1.nrx1 = value,
			// 0xFF12 => self.square1.nrx2 = value,
			// 0xFF13 => self.square1.nrx3 = value,
			// 0xFF14 => self.square1.nrx4 = value,

			// 0xFF15 => self.square2.nrx0 = value, // Unused
			// 0xFF16 => self.square2.nrx1 = value,
			// 0xFF17 => self.square2.nrx2 = value,
			// 0xFF18 => self.square2.nrx3 = value,
			// 0xFF19 => self.square2.nrx4 = value,

			// 0xFF1A => self.wave.nrx0 = value,
			// 0xFF1B => self.wave.nrx1 = value,
			// 0xFF1C => self.wave.nrx2 = value,
			// 0xFF1D => self.wave.nrx3 = value,
			// 0xFF1E => self.wave.nrx4 = value,
			0xFF1F => self.noise.write_nrx0(value), // Unused
			0xFF20 => self.noise.write_nrx1(value),
			0xFF21 => self.noise.write_nrx2(value),
			0xFF22 => self.noise.write_nrx3(value),
			0xFF23 => self.noise.write_nrx4(value),

			0xFF24 => self.nr50 = value, // NR50
			0xFF25 => self.nr51 = value, // NR51

			// 0xFF30..0xFF40 => self.wave.wave_ram[addr as usize - 0xFF30] = value,
			_ => {}
			_ => unreachable!(),
		}
	}
}
