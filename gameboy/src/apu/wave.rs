use serde::{Deserialize, Serialize};

use crate::util::bits::{BIT_6, BIT_7};

use super::{channel::Channel, length_counter::LengthCounter, timer::Timer};

#[repr(u8)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum VolumeCode {
	Zero = 0,
	OneHundred = 1,
	Fifty = 2,
	TwentyFive = 3,
}

impl VolumeCode {
	pub fn shift_amount(self) -> u8 {
		match self {
			VolumeCode::Zero => 4,
			VolumeCode::OneHundred => 0,
			VolumeCode::Fifty => 1,
			VolumeCode::TwentyFive => 2,
		}
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Wave {
	dac_power: bool,
	enabled: bool,
	length_counter: LengthCounter,
	frequency_timer: Timer,
	volume_code: VolumeCode,
	frequency: u16,

	position_counter: u8,
	pub wave_ram: [u8; 32],
}

impl Default for Wave {
	fn default() -> Self {
		Self {
			enabled: false,
			length_counter: LengthCounter::default(),
			frequency_timer: Timer::default(),
			volume_code: VolumeCode::Zero,
			frequency: 0,
			dac_power: false,
			position_counter: 0,
			wave_ram: [0; 32],
		}
	}
}

impl Wave {
	fn timer_period(&self) -> u16 {
		(2048 - self.frequency) * 2
	}

	fn sample(&self, counter_position: u8) -> u8 {
		let sample_index = counter_position / 2;
		let sample = self.wave_ram[sample_index as usize];
		let shift = if counter_position & 1 == 0 { 4 } else { 0 };
		(sample >> shift) & 0xF
	}
}

impl Channel for Wave {
	fn sample_with_volume(&self) -> f32 {
		if !self.dac_power || !self.enabled {
			return 0.0;
		}

		let volume_shift = self.volume_code.shift_amount();
		let sample = (self.sample(self.position_counter) << volume_shift) & 0xF;

		(sample as f32 / 15.0) * 2.0 - 1.0
	}

	fn read_nrx0(&self) -> u8 {
		if self.dac_power {
			BIT_7
		} else {
			0
		}
	}

	fn write_nrx0(&mut self, value: u8) {
		self.dac_power = value & BIT_7 != 0;
	}

	fn write_nrx1(&mut self, value: u8) {
		self.length_counter.reload(value)
	}

	fn read_nrx1(&self) -> u8 {
		self.length_counter.read_length()
	}

	fn write_nrx2(&mut self, value: u8) {
		self.volume_code = match (value & 0b0110_0000) >> 5 {
			0 => VolumeCode::Zero,
			1 => VolumeCode::OneHundred,
			2 => VolumeCode::Fifty,
			3 => VolumeCode::TwentyFive,
			_ => unreachable!(),
		}
	}

	fn read_nrx2(&self) -> u8 {
		(self.volume_code as u8) << 5
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
		let frequency_msb = self.frequency >> 8;
		let trigger = if self.enabled { BIT_7 } else { 0 };
		let length = if self.length_counter.enabled {
			BIT_6
		} else {
			0
		};

		trigger | length | frequency_msb as u8
	}

	fn tick(&mut self) {
		if self.frequency_timer.tick() {
			self.position_counter = (self.position_counter + 1) & 63;
		}
	}

	fn volume(&self) -> u8 {
		0
	}

	fn sample(&self) -> u8 {
		0
	}

	fn enabled(&self) -> bool {
		self.enabled && self.dac_power
	}

	fn reset(&mut self) {
		self.volume_code = VolumeCode::Zero;
		self.enabled = false;
		self.dac_power = false;
		self.frequency = 0;
		self.frequency_timer.reload();
		self.position_counter = 0;
		self.length_counter.enabled = false;
		self.length_counter.reload(0)
	}

	fn tick_length_ctr(&mut self) {
		if self.length_counter.tick() {
			self.enabled = false;
		}
	}

	fn tick_sweep(&mut self) {}
	fn tick_vol_env(&mut self) {}
}
