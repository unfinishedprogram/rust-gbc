use serde::{Deserialize, Serialize};

use crate::util::BigArray;

#[derive(Clone, Serialize, Deserialize)]
pub enum WorkRam {
	Cgb(Box<WorkRamDataCGB>),
	Dmg(Box<WorkRamDataDMG>),
}

pub trait BankedWorkRam {
	fn set_bank_number(&mut self, _bank: u8);
	fn get_bank_number(&self) -> u8;

	fn get_bank(&self, bank: u8) -> &[u8; 0x1000];
	fn get_bank_mut(&mut self, bank: u8) -> &mut [u8; 0x1000];
}

impl BankedWorkRam for WorkRam {
	fn get_bank_number(&self) -> u8 {
		match self {
			WorkRam::Cgb(state) => state.bank,
			WorkRam::Dmg(_) => 1,
		}
	}

	fn get_bank(&self, bank: u8) -> &[u8; 0x1000] {
		match self {
			WorkRam::Cgb(state) => state.get_bank(bank),
			WorkRam::Dmg(state) => state.get_bank(bank),
		}
	}

	fn get_bank_mut(&mut self, bank: u8) -> &mut [u8; 0x1000] {
		match self {
			WorkRam::Cgb(state) => state.get_bank_mut(bank),
			WorkRam::Dmg(state) => state.get_bank_mut(bank),
		}
	}

	fn set_bank_number(&mut self, bank: u8) {
		match self {
			WorkRam::Cgb(state) => state.set_bank_number(bank),
			WorkRam::Dmg(state) => state.set_bank_number(bank),
		}
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WorkRamDataCGB {
	bank: u8,

	#[serde(with = "BigArray")]
	bank_0: [u8; 0x1000],
	#[serde(with = "BigArray")]
	bank_1: [u8; 0x1000],
	#[serde(with = "BigArray")]
	bank_2: [u8; 0x1000],
	#[serde(with = "BigArray")]
	bank_3: [u8; 0x1000],
	#[serde(with = "BigArray")]
	bank_4: [u8; 0x1000],
	#[serde(with = "BigArray")]
	bank_5: [u8; 0x1000],
	#[serde(with = "BigArray")]
	bank_6: [u8; 0x1000],
	#[serde(with = "BigArray")]
	bank_7: [u8; 0x1000],
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WorkRamDataDMG {
	#[serde(with = "BigArray")]
	bank_0: [u8; 0x1000],
	#[serde(with = "BigArray")]
	bank_1: [u8; 0x1000],
}

impl Default for WorkRamDataDMG {
	fn default() -> Self {
		Self {
			bank_0: [0; 0x1000],
			bank_1: [0; 0x1000],
		}
	}
}

impl Default for WorkRamDataCGB {
	fn default() -> Self {
		Self {
			bank: 1,
			bank_0: [0; 0x1000],
			bank_1: [0; 0x1000],
			bank_2: [0; 0x1000],
			bank_3: [0; 0x1000],
			bank_4: [0; 0x1000],
			bank_5: [0; 0x1000],
			bank_6: [0; 0x1000],
			bank_7: [0; 0x1000],
		}
	}
}

impl BankedWorkRam for WorkRamDataDMG {
	fn get_bank_number(&self) -> u8 {
		1
	}
	fn set_bank_number(&mut self, _bank: u8) {}
	fn get_bank(&self, bank: u8) -> &[u8; 0x1000] {
		let bank = bank & 1;
		if bank == 0 {
			&self.bank_0
		} else {
			&self.bank_1
		}
	}

	fn get_bank_mut(&mut self, bank: u8) -> &mut [u8; 0x1000] {
		if bank & 1 == 1 {
			&mut self.bank_1
		} else {
			&mut self.bank_0
		}
	}
}

impl BankedWorkRam for WorkRamDataCGB {
	fn set_bank_number(&mut self, bank: u8) {
		self.bank = (bank) & 3;
		self.bank = self.bank.max(1);
	}

	fn get_bank_number(&self) -> u8 {
		self.bank
	}

	fn get_bank(&self, bank: u8) -> &[u8; 0x1000] {
		match bank {
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

	fn get_bank_mut(&mut self, bank: u8) -> &mut [u8; 0x1000] {
		match bank {
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
