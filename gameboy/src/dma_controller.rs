use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::util::bits::BIT_7;

#[derive(Clone, Copy, Debug)]
pub struct DMATransferRequest {
	pub from: u16,
	pub to: u16,
	pub rows: u16,
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
		self.source |= (value & 0xF0) as u16;
	}

	pub fn write_destination_high(&mut self, value: u8) {
		self.destination &= 0x00FF;
		self.destination |= (((value & 0x1F) as u16) << 8) | 0x8000;
	}

	pub fn write_destination_low(&mut self, value: u8) {
		self.destination &= 0xFF00;
		self.destination |= (value & 0xF0) as u16;
	}

	pub fn read_hdma5(&self) -> u8 {
		self.hdma5
	}

	pub fn get_source(&self) -> u16 {
		self.source
	}

	pub fn get_destination(&self) -> u16 {
		self.destination
	}

	pub fn write_hdma5(&mut self, value: u8) -> Option<DMATransferRequest> {
		log::info!("Wrote to HDMA: {value:04X}");

		let transfer_active = self.hdma5 & BIT_7 == 0;

		// HDMA Active
		if transfer_active && value & BIT_7 == 0 {
			// Terminate HDMA
			log::info!("Write to HDMA caused pause");
			self.hdma5 |= 0x80;
			return None;
		}

		if value & BIT_7 == 0 {
			let rows = ((value & 0x7F) as u16) + 1;
			let req = Some(DMATransferRequest {
				from: self.get_source(),
				to: self.get_destination(),
				rows,
			});

			self.hdma5 = 0xFF;
			self.source += rows * 16;
			self.destination += rows * 16;

			return req;
		} else {
			let rows = value & 0x7F;
			self.hdma5 = rows;
			log::info!("HBlank DMA Transfer Requested");
		}
		None
	}

	// Each step might return a transfer, this transfer must be performed by the caller
	pub fn step(&mut self) -> Option<DMATransferRequest> {
		if self.hdma5 & BIT_7 == 0 {
			log::info!(
				"STEPPING DMA Src:{:X}, Dest:{:X}, HDMA5:{:X}",
				self.get_source(),
				self.get_destination(),
				self.read_hdma5()
			);

			let request = DMATransferRequest {
				from: self.get_source(),
				to: self.get_destination(),
				rows: 1,
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
