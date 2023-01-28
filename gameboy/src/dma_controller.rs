use serde::{Deserialize, Serialize};

use crate::util::bits::BIT_7;

#[derive(Clone, Copy)]
pub struct TransferRequest {
	pub from: u16,
	pub to: u16,
	pub bytes: u16,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
	pub mode: TransferMode,
	pub source: u16,
	pub destination: u16,
	// Length measured in chunks of 16 bytes
	pub total_chunks: u16,
	// Transfer is always done in increments of 16 bytes
	pub bytes_sent: u16,
}

impl Transfer {
	pub fn chunks_remaining(&self) -> u16 {
		self.total_chunks.saturating_sub(self.bytes_sent >> 4)
	}

	pub fn complete(&self) -> bool {
		self.chunks_remaining() == 0
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

	fn new_transfer(&self, byte: u8) -> Transfer {
		let mode = match byte & BIT_7 == BIT_7 {
			true => TransferMode::HBlank,
			false => TransferMode::GeneralPurpose,
		};

		let total_chunks = ((byte & !BIT_7) + 1) as u16;

		let source = self.source & 0xFFF0;
		let destination = (self.destination & 0x1FF0) + 0x8000;
		Transfer {
			mode,
			source,
			destination,
			total_chunks,
			bytes_sent: 0,
		}
	}

	pub fn write_hdma5(&mut self, value: u8) {
		match &mut self.transfer {
			Some(transfer) => {
				if matches!(transfer.mode, TransferMode::GeneralPurpose) {
					panic!("Can't cancel general HDMA");
				}
				// Writing zero to bit 7 when a HDMA transfer is active terminates it
				if value & BIT_7 == 0 {
					self.hdma5 = (transfer.chunks_remaining() - 1) as u8;
					self.hdma5 |= BIT_7;
					self.transfer = None;
				}
			}

			None => self.transfer = Some(self.new_transfer(value)),
		}
	}

	pub fn read_hdma5(&self) -> u8 {
		if let Some(transfer) = &self.transfer {
			(transfer.chunks_remaining() - 1) as u8
		} else {
			return self.hdma5;
		}
	}

	pub fn get_next_transfer(&mut self) -> Option<TransferRequest> {
		let Some(transfer) = &mut self.transfer else { return None };

		if transfer.complete() {
			panic!("Transfer is already complete")
		}

		let byte_offset = transfer.bytes_sent;
		let from = transfer.source + byte_offset;
		let to = transfer.destination + byte_offset;

		let bytes = match transfer.mode {
			TransferMode::GeneralPurpose => 1,
			TransferMode::HBlank => 16,
		};

		transfer.bytes_sent += bytes;
		Some(TransferRequest { from, to, bytes })
	}

	// Each step  might return a transfer, this transfer must be performed by the caller
	pub fn step(&mut self, h_blank: bool) -> Option<TransferRequest> {
		let Some(transfer) = &self.transfer else { return None };

		let request = match transfer.mode {
			TransferMode::GeneralPurpose => self.get_next_transfer(),
			TransferMode::HBlank if h_blank => self.get_next_transfer(),
			_ => return None,
		};

		if let Some(transfer) = &mut self.transfer {
			self.hdma5 = transfer.chunks_remaining() as u8;

			if transfer.complete() {
				// When the transfer is done, HDMA5 should read 0xFF
				self.hdma5 = 0xFF;
				self.transfer = None;
			}
		}

		request
	}
}
