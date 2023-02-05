use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::util::bits::BIT_7;

#[derive(Clone, Copy)]
pub struct TransferRequest {
	pub from: u16,
	pub to: u16,
	pub bytes: u16,
}

impl Debug for TransferRequest {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{:X} -> {:X}, size:{:X}",
			&self.from, &self.to, &self.bytes
		)
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DMAController {
	source: u16,      // HDMA1 | HDMA2
	destination: u16, // HDMA3 | HDMA4
	hdma5: u8,
}

impl Default for DMAController {
	fn default() -> Self {
		Self {
			source: 0,
			destination: 0,
			hdma5: 0xFF,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferMode {
	GeneralPurpose,
	HBlank,
}

impl DMAController {
	pub fn write_source_high(&mut self, value: u8) {
		self.source &= 0x00FF;
		self.source |= (value as u16) << 8;
	}
	pub fn write_source_low(&mut self, value: u8) {
		self.source &= 0xFF00;
		self.source |= value as u16;
	}
	pub fn write_destination_high(&mut self, value: u8) {
		self.destination &= 0x00FF;
		self.destination |= (value as u16) << 8;
	}
	pub fn write_destination_low(&mut self, value: u8) {
		self.destination &= 0xFF00;
		self.destination |= value as u16;
	}

	pub fn read_hdma5(&self) -> u8 {
		self.hdma5
	}

	pub fn get_source(&self) -> u16 {
		self.source & 0xFFF0
	}

	pub fn get_destination(&self) -> u16 {
		(self.destination & 0xFFF0) | 0x8000
	}

	pub fn write_hdma5(&mut self, value: u8) -> Option<TransferRequest> {
		log::info!("Wrote to HDMA");

		// GDMA Transfer
		if value & BIT_7 == 0 {
			// HDMA Active
			if self.hdma5 & BIT_7 == 0 {
				self.hdma5 |= 0x80;
				log::info!("transfer canceled");
			} else {
				let bytes = (((value & 0x7F) as u16) + 1) * 0x10;
				self.hdma5 = 0xFF;
				return Some(TransferRequest {
					from: self.get_source(),
					to: self.get_destination(),
					bytes,
				});
			}
		} else {
			self.hdma5 = value & 0x7F;
		}
		None
	}

	// Each step  might return a transfer, this transfer must be performed by the caller
	pub fn step(&mut self) -> Option<TransferRequest> {
		if self.hdma5 & BIT_7 == 0 {
			let request = TransferRequest {
				from: self.get_source(),
				to: self.get_destination(),
				bytes: 0x10,
			};

			self.source += 0x10;
			self.destination += 0x10;
			self.hdma5 = self.hdma5.wrapping_sub(1);
			Some(request)
		} else {
			None
		}
	}
}
