use crate::{
	apu::frame_sequencer,
	util::bits::{BIT_3, BIT_6, BIT_7},
};
use serde::{Deserialize, Serialize};

use super::super::{
	channel::Channel, length_counter::LengthCounter, lfsr::Lfsr, timer::Timer,
	volume_envelope::VolumeEnvelope,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Noise {
	lfsr: Lfsr,
	volume_envelope: VolumeEnvelope,
	enabled: bool,

	clock_shift: u8,
	devisor_code: u8,

	frequency_timer: Timer,
	length_counter: LengthCounter,
	acc: u32,
}

impl Noise {
	fn timer_period(&self) -> u16 {
		self.divisor() << self.clock_shift
	}

	fn divisor(&self) -> u16 {
		match self.devisor_code & 7 {
			0 => 8,
			1 => 16,
			2 => 32,
			3 => 48,
			4 => 64,
			5 => 80,
			6 => 96,
			7 => 112,
			_ => unreachable!(),
		}
	}
}

impl Default for Noise {
	fn default() -> Self {
		let mut res = Self {
			lfsr: Lfsr::default(),
			volume_envelope: VolumeEnvelope::default(),
			enabled: false,
			clock_shift: 0,
			devisor_code: 0,
			frequency_timer: Timer::new(0),
			length_counter: LengthCounter::default(),
			acc: 0,
		};
		res.frequency_timer.set_period(res.timer_period());
		res
	}
}

impl Channel for Noise {
	fn write_nrx0(&mut self, _value: u8) {}
	fn read_nrx0(&self) -> u8 {
		0xFF
	}

	fn write_nrx1(&mut self, value: u8) {
		self.length_counter.reload(value & 0b0011_1111);
	}
	fn read_nrx1(&self) -> u8 {
		self.length_counter.read_length()
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
		self.clock_shift = (value >> 4) & 0b1111;
		self.devisor_code = value & 0b111;

		self.frequency_timer.set_period(self.timer_period());

		self.lfsr.width = if value & BIT_3 == BIT_3 { 6 } else { 14 };
		self.lfsr.reset();
	}

	fn read_nrx3(&self) -> u8 {
		let lfsr_mode = if self.lfsr.width == 6 { BIT_3 } else { 0 };
		self.clock_shift << 4 | lfsr_mode | self.devisor_code
	}

	fn write_nrx4(&mut self, value: u8, _next_frame_sequencer_result: frame_sequencer::TickResult) {
		let trigger = value & BIT_7 == BIT_7;

		self.length_counter.set_enabled(value & BIT_6 == BIT_6);
		if trigger {
			self.enabled = true;

			self.frequency_timer.reload();
			self.volume_envelope.reload();
			self.lfsr.reset();
		}
	}
	fn read_nrx4(&self) -> u8 {
		let length_enable = (self.length_counter.enabled() as u8) << 6;
		let trigger = (self.enabled as u8) << 7;
		length_enable | trigger
	}

	fn tick_length_ctr(&mut self) {
		if self.length_counter.tick() {
			self.enabled = false;
		}
	}

	fn tick_vol_env(&mut self) {
		self.volume_envelope.tick();
	}

	fn tick(&mut self) {
		if self.frequency_timer.tick() {
			self.lfsr.step();
		}
	}

	fn enabled(&self) -> bool {
		self.enabled && self.volume_envelope.dac_enabled()
	}

	fn sample(&self) -> u8 {
		(!self.lfsr.shift_register & 1) as u8
	}

	fn reset(&mut self) {
		self.lfsr.reset();
		self.lfsr.width = 14;
		self.clock_shift = 0;
		self.devisor_code = 0;

		self.length_counter.reload(0);
		self.volume_envelope.write_byte(0);
		self.length_counter.set_enabled(false);
		self.enabled = false;
	}

	fn volume(&self) -> u8 {
		self.volume_envelope.volume
	}

	fn tick_sweep(&mut self) {}
}
