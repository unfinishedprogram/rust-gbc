use crate::util::bits::{BIT_6, BIT_7};

use super::{
	channel::Channel, length_counter::LengthCounter, timer::Timer, volume_envelope::VolumeEnvelope,
};

use serde::{Deserialize, Serialize};

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
}

impl Square {
	const DUTY: [u8; 4] = [0b00000001, 0b10000001, 0b10000111, 0b01111110];

	fn timer_period(&self) -> u16 {
		(2048 - self.frequency) * 4
	}
}

impl Channel for Square {
	fn write_nrx0(&mut self, _value: u8) {
		if self.sweeper {}
	}
	fn read_nrx0(&self) -> u8 {
		if self.sweeper {}
		0
	}

	fn write_nrx1(&mut self, value: u8) {
		// NR11 FF11 DDLL LLLL Duty, Length load (64-L)
		self.length_counter.reload(value & 0b0011_1111)
	}

	fn read_nrx1(&self) -> u8 {
		// NR11 FF11 DDLL LLLL Duty, Length load (64-L)
		self.length_counter.length
	}

	fn write_nrx2(&mut self, value: u8) {
		self.volume_envelope.write_byte(value)
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

	fn write_nrx4(&mut self, value: u8) {
		let trigger = value & BIT_7 == BIT_7;
		self.length_counter.enabled = value & BIT_6 == BIT_6;
		if trigger {
			self.enabled = true;
		}

		let frequency_msb = value & 0b111;
		self.frequency &= 0x00FF;
		self.frequency |= (frequency_msb as u16) << 8;
		self.frequency_timer.set_period(self.timer_period());
	}

	fn read_nrx4(&self) -> u8 {
		let trigger = (self.enabled as u8) << 7;
		let length_enable = (self.length_counter.enabled as u8) << 6;
		let frequency_msb = (self.frequency >> 8) as u8;
		trigger | length_enable | frequency_msb
	}

	fn tick(&mut self) {
		if self.frequency_timer.tick() {
			self.duty_cycle = (self.duty_cycle + 1) & 0x7;
		}
	}

	fn sample(&mut self) -> f32 {
		let volume = self.volume_envelope.volume;
		let duty = (Self::DUTY[self.duty_index as usize] & (1 << self.duty_cycle) != 0) as u8;
		let dac_input = duty * volume;
		(dac_input as f32 / 15.0) * 2.0
	}

	fn enabled(&self) -> bool {
		self.enabled
	}
}
