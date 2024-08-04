use serde::{Deserialize, Serialize};

use crate::{
	apu::frame_sequencer,
	util::bits::{BIT_6, BIT_7},
};

use super::super::{
	channel::Channel, length_counter::LengthCounter, sweep::Sweep, timer::Timer,
	volume_envelope::VolumeEnvelope,
};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Square {
	sweeper: bool,
	enabled: bool,

	length_counter: LengthCounter,

	volume_envelope: VolumeEnvelope,

	frequency_timer: Timer,
	frequency: u16,

	duty_cycle: u8,
	duty_index: u8,

	sweep: Sweep,
}

impl Square {
	const DUTY: [u8; 4] = [0b00000001, 0b10000001, 0b10000111, 0b01111110];

	fn timer_period(&self) -> u16 {
		(2048 - self.frequency) * 4
	}

	pub fn sweeper() -> Self {
		Self {
			sweeper: true,
			..Self::default()
		}
	}
}

impl Channel for Square {
	fn write_nrx0(&mut self, value: u8) {
		if self.sweeper {
			self.sweep.write_byte(value);
		}
	}
	fn read_nrx0(&self) -> u8 {
		if self.sweeper {
			self.sweep.read_byte()
		} else {
			0
		}
	}

	fn write_nrx1(&mut self, value: u8) {
		self.length_counter.reload(value & 0b0011_1111);
		self.duty_index = (value >> 6) & 0b11;
	}

	fn read_nrx1(&self) -> u8 {
		(self.duty_index << 6) | self.length_counter.read_length()
	}

	fn write_nrx2(&mut self, value: u8) {
		self.volume_envelope.write_byte(value);
		if !self.volume_envelope.dac_enabled() {
			self.enabled = false;
		}
	}

	fn read_nrx2(&self) -> u8 {
		self.volume_envelope.read_byte()
	}

	fn write_nrx3(&mut self, value: u8) {
		// Frequency LSB
		self.frequency &= 0x0700;
		self.frequency |= value as u16;
		self.frequency_timer.set_period(self.timer_period());
	}

	fn read_nrx3(&self) -> u8 {
		(self.frequency & 0x00FF) as u8
	}

	fn write_nrx4(&mut self, value: u8, next_frame_sequencer_result: frame_sequencer::TickResult) {
		let trigger = value & BIT_7 == BIT_7;
		let length_enable = value & BIT_6 == BIT_6;

		// If the channel is triggered, and the frame-sequencer is in the first half of the length period
		// Then the length counter is clocked

		let length_previously_enabled = self.length_counter.enabled();
		self.length_counter.set_enabled(length_enable);

		if trigger {
			self.length_counter.unfreeze();
			self.enabled = true;
			if self.sweeper {
				self.sweep.trigger(self.frequency);
			}

			let frequency_msb = value & 0b111;
			self.frequency &= 0x00FF;
			self.frequency |= (frequency_msb as u16) << 8;
			self.frequency_timer.set_period(self.timer_period());
		}

		let in_first_half = match next_frame_sequencer_result {
			frame_sequencer::TickResult::None => true,
			frame_sequencer::TickResult::LengthCtrl => false,
			frame_sequencer::TickResult::VolumeEnv => true,
			frame_sequencer::TickResult::LengthCtrlAndSweep => false,
		};

		let mut clock_length_extra = !length_previously_enabled && self.length_counter.enabled();
		clock_length_extra &= in_first_half;

		if clock_length_extra {
			self.tick_length_ctr();
		}
	}

	fn read_nrx4(&self) -> u8 {
		let trigger = (self.enabled as u8) << 7;
		let length_enable = (self.length_counter.enabled() as u8) << 6;
		let frequency_msb = (self.frequency >> 8) as u8;
		trigger | length_enable | frequency_msb
	}

	fn tick(&mut self) {
		if self.frequency_timer.tick() {
			self.duty_cycle = (self.duty_cycle + 1) & 0x7;
		}
	}

	fn volume(&self) -> u8 {
		self.volume_envelope.volume
	}

	fn sample(&self) -> u8 {
		(Self::DUTY[self.duty_index as usize] & (1 << self.duty_cycle) != 0) as u8
	}

	fn enabled(&self) -> bool {
		self.enabled && self.volume_envelope.dac_enabled()
	}

	fn reset(&mut self) {
		if self.sweeper {
			self.sweep.write_byte(0);
		}
		self.duty_cycle = 0;
		self.duty_index = 0;
		self.volume_envelope.write_byte(0);
		self.length_counter.set_enabled(false);
		self.length_counter.reload(0);
		self.enabled = false;
	}

	fn tick_sweep(&mut self) {
		if self.sweeper {
			let (disable, new_freq) = self.sweep.tick();

			if disable {
				self.enabled = false;
			}

			if let Some(new_freq) = new_freq {
				self.frequency = new_freq;
			}
		}
	}

	fn tick_length_ctr(&mut self) {
		if self.length_counter.tick() {
			self.enabled = false;
		}
	}

	fn tick_vol_env(&mut self) {
		self.volume_envelope.tick();
	}
}
