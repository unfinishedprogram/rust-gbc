use serde::{Deserialize, Serialize};

use crate::util::BigArray;

const BANK_SIZE: usize = 0x1000;

#[derive(Clone, Serialize, Deserialize)]
pub enum WorkRam {
	Cgb(Box<WorkRamDataCGB>),
	Dmg(Box<WorkRamDataDMG>),
}

impl WorkRam {
	fn inner(&self) -> &dyn BankedWorkRam {
		match self {
			WorkRam::Cgb(state) => state.as_ref(),
			WorkRam::Dmg(state) => state.as_ref(),
		}
	}

	fn inner_mut(&mut self) -> &mut dyn BankedWorkRam {
		match self {
			WorkRam::Cgb(state) => state.as_mut(),
			WorkRam::Dmg(state) => state.as_mut(),
		}
	}
}

impl<'a> From<&'a WorkRam> for &'a dyn BankedWorkRam {
	fn from(wram: &'a WorkRam) -> Self {
		wram.inner()
	}
}

impl<'a> From<&'a mut WorkRam> for &'a mut dyn BankedWorkRam {
	fn from(wram: &'a mut WorkRam) -> Self {
		wram.inner_mut()
	}
}

pub trait BankedWorkRam {
	fn set_bank_number(&mut self, _bank: u8) {}
	fn get_bank_number(&self) -> u8 {
		1
	}

	fn get_high_bank(&self) -> &[u8; BANK_SIZE];
	fn get_low_bank(&self) -> &[u8; BANK_SIZE];

	fn get_high_bank_mut(&mut self) -> &mut [u8; BANK_SIZE];
	fn get_low_bank_mut(&mut self) -> &mut [u8; BANK_SIZE];
}

impl BankedWorkRam for WorkRam {
	fn set_bank_number(&mut self, bank: u8) {
		self.inner_mut().set_bank_number(bank);
	}

	fn get_bank_number(&self) -> u8 {
		self.inner().get_bank_number()
	}

	fn get_high_bank(&self) -> &[u8; BANK_SIZE] {
		self.inner().get_high_bank()
	}

	fn get_low_bank(&self) -> &[u8; BANK_SIZE] {
		self.inner().get_low_bank()
	}

	fn get_high_bank_mut(&mut self) -> &mut [u8; BANK_SIZE] {
		self.inner_mut().get_high_bank_mut()
	}

	fn get_low_bank_mut(&mut self) -> &mut [u8; BANK_SIZE] {
		self.inner_mut().get_low_bank_mut()
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WorkRamDataCGB {
	bank: u8,

	#[serde(with = "BigArray")]
	bank_0: [u8; BANK_SIZE],
	#[serde(with = "BigArray")]
	bank_1: [u8; BANK_SIZE],
	#[serde(with = "BigArray")]
	bank_2: [u8; BANK_SIZE],
	#[serde(with = "BigArray")]
	bank_3: [u8; BANK_SIZE],
	#[serde(with = "BigArray")]
	bank_4: [u8; BANK_SIZE],
	#[serde(with = "BigArray")]
	bank_5: [u8; BANK_SIZE],
	#[serde(with = "BigArray")]
	bank_6: [u8; BANK_SIZE],
	#[serde(with = "BigArray")]
	bank_7: [u8; BANK_SIZE],
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WorkRamDataDMG {
	#[serde(with = "BigArray")]
	bank_0: [u8; BANK_SIZE],
	#[serde(with = "BigArray")]
	bank_1: [u8; BANK_SIZE],
}

impl Default for WorkRamDataDMG {
	fn default() -> Self {
		Self {
			bank_0: [0; BANK_SIZE],
			bank_1: [0; BANK_SIZE],
		}
	}
}

impl Default for WorkRamDataCGB {
	fn default() -> Self {
		Self {
			bank: 1,
			bank_0: [0; BANK_SIZE],
			bank_1: [0; BANK_SIZE],
			bank_2: [0; BANK_SIZE],
			bank_3: [0; BANK_SIZE],
			bank_4: [0; BANK_SIZE],
			bank_5: [0; BANK_SIZE],
			bank_6: [0; BANK_SIZE],
			bank_7: [0; BANK_SIZE],
		}
	}
}

impl BankedWorkRam for WorkRamDataDMG {
	fn get_low_bank(&self) -> &[u8; BANK_SIZE] {
		&self.bank_0
	}

	fn get_high_bank(&self) -> &[u8; BANK_SIZE] {
		&self.bank_1
	}

	fn get_low_bank_mut(&mut self) -> &mut [u8; BANK_SIZE] {
		&mut self.bank_0
	}

	fn get_high_bank_mut(&mut self) -> &mut [u8; BANK_SIZE] {
		&mut self.bank_1
	}
}

impl BankedWorkRam for WorkRamDataCGB {
	fn set_bank_number(&mut self, bank: u8) {
		self.bank = ((bank) & 0b111).max(1);
	}

	fn get_bank_number(&self) -> u8 {
		self.bank
	}

	fn get_low_bank(&self) -> &[u8; BANK_SIZE] {
		&self.bank_0
	}

	fn get_low_bank_mut(&mut self) -> &mut [u8; BANK_SIZE] {
		&mut self.bank_0
	}

	fn get_high_bank(&self) -> &[u8; BANK_SIZE] {
		match self.get_bank_number() {
			0 => &self.bank_0,
			1 => &self.bank_1,
			2 => &self.bank_2,
			3 => &self.bank_3,
			4 => &self.bank_4,
			5 => &self.bank_5,
			6 => &self.bank_6,
			7 => &self.bank_7,
			_ => unreachable!(),
		}
	}

	fn get_high_bank_mut(&mut self) -> &mut [u8; BANK_SIZE] {
		match self.get_bank_number() {
			0 => &mut self.bank_0,
			1 => &mut self.bank_1,
			2 => &mut self.bank_2,
			3 => &mut self.bank_3,
			4 => &mut self.bank_4,
			5 => &mut self.bank_5,
			6 => &mut self.bank_6,
			7 => &mut self.bank_7,
			_ => unreachable!(),
		}
	}
}
