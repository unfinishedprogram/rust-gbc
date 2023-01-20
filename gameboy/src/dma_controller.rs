use std::assert_matches::debug_assert_matches;

use serde::{Deserialize, Serialize};

use crate::{
	cartridge::memory_bank_controller::Cartridge,
	memory_mapper::MemoryMapper,
	ppu::{VRAMBank, PPU},
	util::bits::BIT_7,
	work_ram::BankedWorkRam,
};

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
	pub chunks_sent: u16,
}

impl Transfer {
	pub fn chunks_remaining(&self) -> u16 {
		self.total_chunks - self.chunks_sent
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
		let destination = self.destination & 0x1FF0;

		debug_assert_matches!(source, (0x0000..0x7FF0) | (0xA000..0xDFF0));
		// debug_assert_matches!(destination, 0x0..8176);

		Transfer {
			mode,
			source,
			destination,
			total_chunks,
			chunks_sent: 0,
		}
	}

	pub fn write_hdma5(&mut self, value: u8) {
		match &mut self.transfer {
			Some(transfer) => {
				// Writing zero to bit 7 when a HDMA transfer is active terminates it
				if value & BIT_7 == 0 {
					self.hdma5 = (transfer.chunks_remaining()) as u8;
					self.hdma5 |= BIT_7;
					self.transfer = None;
				}
			}

			None => self.transfer = Some(self.new_transfer(value)),
		}
	}

	pub fn read_hdma5(&self) -> u8 {
		return self.hdma5;
	}

	pub fn transfer_next_chunk(
		&mut self,
		ppu: &mut PPU,
		cartridge: &mut Option<Cartridge>,
		wram: &mut impl BankedWorkRam,
		v_ram_bank: &VRAMBank,
	) {
		let Some(transfer) = &mut self.transfer else { return };
		if transfer.complete() {
			panic!("Transfer is already complete")
		}

		let vram = match v_ram_bank {
			VRAMBank::Bank0 => &mut ppu.v_ram_bank_0,
			VRAMBank::Bank1 => &mut ppu.v_ram_bank_1,
		};

		let byte_offset = transfer.chunks_sent * 16;

		for i in 0..16 {
			let src_addr = (transfer.source + byte_offset + i) as usize;
			let dest_addr = (transfer.destination + byte_offset + i) as usize;
			match src_addr {
				// Cartage address
				0x0000..0x8000 | 0xA000..0xC000 => {
					if let Some(cartridge) = cartridge {
						vram[dest_addr] = cartridge.read(src_addr as u16)
					}
				}
				0xC000..0xD000 => vram[dest_addr] = wram.get_bank(0)[src_addr - 0xC000],
				0xD000..0xE000 => {
					vram[dest_addr] = wram.get_bank(wram.get_bank_number())[src_addr - 0xD000]
				}
				_ => unreachable!("DMA outside range"),
			}
		}

		transfer.chunks_sent += 1;
	}

	pub fn step_controller(
		&mut self,
		ppu: &mut PPU,
		cartridge: &mut Option<Cartridge>,
		wram: &mut impl BankedWorkRam,
		v_ram_bank: &VRAMBank,
	) {
		let (length, mode) = {
			let Some(transfer) = &self.transfer else { return };
			(transfer.total_chunks, &transfer.mode)
		};

		match mode {
			TransferMode::GeneralPurpose => {
				for _ in 0..length {
					self.transfer_next_chunk(ppu, cartridge, wram, v_ram_bank)
				}
			}
			TransferMode::HBlank => {
				self.transfer_next_chunk(ppu, cartridge, wram, v_ram_bank);
			}
		}

		if let Some(transfer) = &mut self.transfer {
			self.hdma5 = transfer.chunks_remaining() as u8;

			if transfer.complete() {
				// When the transfer is done, HDMA5 should read 0xFF
				self.hdma5 = 0xFF;
				self.transfer = None;
			}
		}
	}
}
