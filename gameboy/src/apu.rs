mod channel;
mod frame_sequencer;
mod length_counter;
mod lfsr;
mod noise;
mod square;
mod sweep;
mod timer;
mod volume_envelope;

use crate::{
	cgb::Speed,
	sm83::memory_mapper::MemoryMapper,
	util::bits::{falling_edge, BIT_5, BIT_6},
};

use serde::{Deserialize, Serialize};

use self::{channel::Channel, frame_sequencer::FrameSequencer, noise::Noise, square::Square};

// Audio Processing Unit
// https://gbdev.io/pandocs/Audio_details.html#audio-details
#[derive(Clone, Serialize, Deserialize)]
pub struct Apu {
	prev_div: u8,
	frame_sequencer: FrameSequencer,

	square1: Square,
	square2: Square,
	// wave: Wave,
	noise: Noise,

	// Master Volume
	nr50: u8,
	nr51: u8,
	power_on: bool,
}

// There are 4 sound channels each with a generator and a DAC
// Each generator produces values from 0 to 15 or 0x0-0XF
// The DAC then translates this into an "analog" value between -1 and 1

// The four analog channel outputs are then fed into the mixer, which selectively adds them (depending on NR51)
// into two analog outputs (Left and Right). Thus, the analog range of those outputs is 4× that of each channel, -4 to 4.
// Then these final outputs are scaled based on NR50 and output to the speakers.
// NOTE: this scaling can never silence a non-silent input.

// TODO: Implement PCM registers CGB only

impl Apu {
	pub fn step_t_state(&mut self, div: u8, speed: Speed) {
		if !self.power_on {
			self.prev_div = div;
			return;
		}

		// 512hz timer
		let increment_clock = {
			let div_bit_mask = match speed {
				Speed::Normal => BIT_5,
				Speed::Double => BIT_6,
			};

			let res = falling_edge(self.prev_div, div, div_bit_mask);
			self.prev_div = div;
			res
		};

		self.square1.tick();
		self.square2.tick();
		self.noise.tick();

		if increment_clock {
			self.step_frame_sequencer();
		}
	}

	// Ticked on the falling edge of the div register's 5th bit (6th bit in double speed mode)
	// This should tick at 512hz
	fn step_frame_sequencer(&mut self) {
		if let Some(res) = self.frame_sequencer.tick() {
			use frame_sequencer::TickResult as R;
			match res {
				R::LengthCtrl => self.tick_length_ctr(),
				R::VolumeEnv => self.tick_vol_env(),
				R::LengthCtrlAndSweep => {
					self.tick_length_ctr();
					self.tick_sweep();
				}
			}
		}
	}

	fn tick_sweep(&mut self) {
		self.square1.tick_sweep();
	}

	fn tick_length_ctr(&mut self) {
		self.square1.tick_length_ctr();
		self.square2.tick_length_ctr();
		self.noise.tick_length_ctr();
	}
	fn tick_vol_env(&mut self) {
		self.square1.tick_vol_env();
		self.square2.tick_vol_env();
		self.noise.tick_vol_env();
	}

	fn master_volume(&self) -> (f32, f32) {
		let left = self.nr50 & 0b111;
		let right = (self.nr50 >> 4) & 0b111;

		let left = (left + 1) as f32 / 15.0;
		let right = (right + 1) as f32 / 15.0;

		(left, right)
	}

	fn channel_enabled_lr(&self, channel_idx: u8) -> (f32, f32) {
		let left = ((self.nr51 & (1 << channel_idx) != 0) as u8) as f32;
		let right = ((self.nr51 & (1 << (channel_idx + 4)) != 0) as u8) as f32;

		(left, right)
	}

	fn dac(sample: u8) -> f32 {
		sample as f32 / 15.0
		// (sample as f32 - 7.0) / 15.0
	}

	fn sample_mixer(&mut self) -> (f32, f32) {
		let square1 = Self::dac(self.square1.sample());
		let square2 = Self::dac(self.square2.sample());
		let noise = Self::dac(self.noise.sample());

		let (sq1_l, sq1_r) = self.channel_enabled_lr(0);
		let (sq2_l, sq2_r) = self.channel_enabled_lr(1);
		let (n_l, n_r) = self.channel_enabled_lr(3);

		let left = (sq1_l * square1) + (sq2_l * square2) + (n_l * noise);
		let right = (sq1_r * square1) + (sq2_r * square2) + (n_r * noise);

		(left / 4.0, right / 4.0)
	}

	pub fn sample(&mut self) -> (f32, f32) {
		let (v_left, v_right) = self.master_volume();
		let (left, right) = self.sample_mixer();

		(left * v_left, right * v_right)
	}

	fn set_power_on(&mut self, state: bool) {
		self.power_on = state;
		if !state {
			self.square1.reset();
			self.square2.reset();
			self.noise.reset();
			self.nr50 = 0;
			self.nr51 = 0;
		}
	}

	fn read_nr53(&self) -> u8 {
		let p_on = (self.power_on as u8) << 7;
		let square1 = self.square1.enabled() as u8;
		let square2 = (self.square2.enabled() as u8) << 1;
		let wave = 0;
		let noise = (self.noise.enabled() as u8) << 3;

		p_on | square1 | square2 | wave | noise
	}

	fn read_pcm_12(&self) -> u8 {
		(self.square2.sample() << 4) | self.square1.sample()
	}

	fn read_pcm_34(&self) -> u8 {
		self.noise.sample() << 4
	}
}

impl Default for Apu {
	fn default() -> Self {
		Self {
			power_on: false,
			prev_div: 0,
			frame_sequencer: FrameSequencer::default(),

			square1: Square::default(),
			square2: Square::default(),
			// wave: Wave::default(),
			noise: Noise::new(),

			nr50: 0,
			nr51: 0,
		}
	}
}

impl MemoryMapper for Apu {
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

			_ => 0x00,
		};

		let value = match addr {
			0xFF10 => self.square1.read_nrx0(),
			0xFF11 => self.square1.read_nrx1(),
			0xFF12 => self.square1.read_nrx2(),
			0xFF13 => self.square1.read_nrx3(),
			0xFF14 => self.square1.read_nrx4(),

			0xFF15 => self.square2.read_nrx0(), // Unused
			0xFF16 => self.square2.read_nrx1(),
			0xFF17 => self.square2.read_nrx2(),
			0xFF18 => self.square2.read_nrx3(),
			0xFF19 => self.square2.read_nrx4(),

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

			0xFF24 => self.nr50,        // NR50
			0xFF25 => self.nr51,        // NR51
			0xFF26 => self.read_nr53(), // NR52

			0xFF76 => self.read_pcm_12(),
			0xFF77 => self.read_pcm_34(),

			// 0xFF30..0xFF40 => self.wave.wave_ram[addr as usize - 0xFF30],
			_ => {
				log::error!("Apu read from unhandled address: {:#X}", addr);
				0xFF
			}
		};

		value | unused_mask
	}

	fn write(&mut self, addr: u16, value: u8) {
		if !self.power_on && addr != 0xFF26 {
			return;
		}

		match addr {
			0xFF10 => self.square1.write_nrx0(value),
			0xFF11 => self.square1.write_nrx1(value),
			0xFF12 => self.square1.write_nrx2(value),
			0xFF13 => self.square1.write_nrx3(value),
			0xFF14 => self.square1.write_nrx4(value),

			0xFF15 => self.square2.write_nrx0(value),
			0xFF16 => self.square2.write_nrx1(value),
			0xFF17 => self.square2.write_nrx2(value),
			0xFF18 => self.square2.write_nrx3(value),
			0xFF19 => self.square2.write_nrx4(value),

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

			0xFF24 => self.nr50 = value,                           // NR50
			0xFF25 => self.nr51 = value,                           // NR51
			0xFF26 => self.set_power_on(value & 0b1000_0000 != 0), // NR52

			// 0xFF30..0xFF40 => self.wave.wave_ram[addr as usize - 0xFF30] = value,
			_ => log::error!("Apu write to unhandled address: {:#X}", addr),
		}
	}
}
