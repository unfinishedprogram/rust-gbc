use std::fmt::Debug;

use log::debug;
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

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct DMAController {
	pub transfer: Option<Transfer>,
	source: u16,      // HDMA1 | HDMA2
	destination: u16, // HDMA3 | HDMA4
	hdma5: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferMode {
	GeneralPurpose,
	HBlank,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Transfer {
	pub mode: TransferMode,
	pub source: u16,
	pub destination: u16,
	// Length measured in chunks of 16 bytes
	pub total_chunks: u16,
	// Transfer is always done in increments of 16 bytes
	pub chunks_remaining: u8,
}

impl Transfer {
	pub fn chunks_remaining(&self) -> u16 {
		self.chunks_remaining as u16
	}

	pub fn complete(&self) -> bool {
		self.chunks_remaining() == 0
	}
}

impl Debug for Transfer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let Transfer {
			mode,
			source,
			destination,
			total_chunks,
			chunks_remaining:_,
		} = &self;

		write!(
			f,
			"Mode:{mode:?}\n [{source:X}] -> [{destination:X}]\n Chunks:{total_chunks}"
		)
	}
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

	pub fn gdma_active(&self) -> bool {
		if let Some(transfer) = &self.transfer {
			matches!(transfer.mode, TransferMode::GeneralPurpose)
		} else {
			false
		}
	}

	pub fn dma_is_active(&self) -> bool {
		self.hdma5 & BIT_7 == 0
	}

	pub fn is_hdma_active(&self) -> bool {
		self.hdma5 & 0x80 == 0
	}

	fn new_transfer(source: u16, destination: u16, byte: u8) -> Transfer {
		let mode = match byte & BIT_7 == BIT_7 {
			true => TransferMode::HBlank,
			false => TransferMode::GeneralPurpose,
		};
		debug!("New Transfer: {mode:?}",);
		let total_chunks = ((byte + 1) & !BIT_7) as u16;

		let source = source & 0xFFF0;
		let destination = (destination & 0xFFF0) | 0x8000;

		Transfer {
			mode,
			source,
			destination,
			total_chunks,
			chunks_remaining: total_chunks as u8,
		}
	}

	pub fn on_hdma5_write(&mut self, value: u8) {
		if (value & 0x80) == 0 {
			if self.is_hdma_active() {
				self.transfer = None;
				self.hdma5 |= 0x80;
			}
		} else {
			// save the len
			self.hdma5 = value & 0x7F;
		}
	}

	pub fn write_hdma5(&mut self, value: u8) {
		match &mut self.transfer {
			Some(transfer) => {
				log::error!("Wrote to HDMA");
				debug_assert!(matches!(transfer.mode, TransferMode::HBlank));

				// Writing zero to bit 7 when a HDMA transfer is active terminates it
				if value & BIT_7 == 0 {
					self.hdma5 = transfer.chunks_remaining - 1;
					self.hdma5 |= BIT_7;
					self.transfer = None;
				} else {
					self.hdma5 = value;
					transfer.chunks_remaining = (self.hdma5 & !BIT_7) + 1;
				}
			}

			None => {
				self.transfer = Some(DMAController::new_transfer(
					self.source,
					self.destination,
					value,
				))
			}
		}
		log::debug!("{:b}", value);
		log::debug!("{:?}", self.transfer);
	}

	pub fn read_hdma5(&self) -> u8 {
		if let Some(transfer) = &self.transfer {
			transfer.chunks_remaining - 1
		} else {
			return self.hdma5;
		}
	}

	pub fn get_next_transfer(&mut self) -> Option<TransferRequest> {
		let Some(transfer) = &mut self.transfer else { return None };

		if transfer.complete() {
			panic!("Transfer is already complete")
		}

		let from = transfer.source;
		let to = transfer.destination;

		transfer.source += 16;
		transfer.destination += 16;
		transfer.chunks_remaining -= 1;

		self.hdma5 = transfer.chunks_remaining.wrapping_sub(1);
		if transfer.complete() {
			self.transfer = None;
		}

		Some(TransferRequest {
			from,
			to,
			bytes: 16,
		})
	}

	// Each step  might return a transfer, this transfer must be performed by the caller
	pub fn step(&mut self, h_blank: bool) -> Option<TransferRequest> {
		let Some(transfer) = &self.transfer else { return None };

		match transfer.mode {
			TransferMode::GeneralPurpose => self.get_next_transfer(),
			TransferMode::HBlank if h_blank => self.get_next_transfer(),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;
	#[test]
	fn test_transfer() {
		let transfer = DMAController::new_transfer(0, 0, 0b10000000);
		assert_eq!(transfer.total_chunks, 1);
	}
}
