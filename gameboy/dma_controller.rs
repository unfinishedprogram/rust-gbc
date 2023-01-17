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
	pub total_length: u16,
	pub bytes_sent: u16,
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

	pub fn write_start(&mut self, value: u8) {
		let mode = if value & BIT_7 == BIT_7 {
			TransferMode::GeneralPurpose
		} else {
			TransferMode::HBlank
		};

		let total_length = ((value & !BIT_7) + 1) as u16 * 0x10;

		let source = self.source & 0xFFF0;
		let destination = self.destination & 0x1FF0;

		if !matches!(source, (0x0000..0x7FF0) | (0xA000..0xDFF0)) {
			panic!("{:x}", source)
		}
		// debug_assert!(matches!(destination, 0x8000..0x9FF0));

		self.transfer = Some(Transfer {
			mode,
			source,
			destination,
			total_length,
			bytes_sent: 0,
		});
	}

	pub fn read_length(&self) -> u8 {
		let Some(transfer) = &self.transfer else {
            return !BIT_7
        };

		let length = transfer.total_length - transfer.bytes_sent;
		let length = (length / 0x10 - 1) as u8;
		length | BIT_7
	}

	pub fn step_controller(
		&mut self,
		ppu: &mut PPU,
		cartridge: &mut Option<Cartridge>,
		wram: &mut impl BankedWorkRam,
		v_ram_bank: &VRAMBank,
	) {
		if let Some(transfer) = &mut self.transfer {
			let vram = match v_ram_bank {
				VRAMBank::Bank0 => &mut ppu.v_ram_bank_0,
				VRAMBank::Bank1 => &mut ppu.v_ram_bank_1,
			};

			for i in 0..transfer.total_length {
				let src_addr = transfer.source + i;
				let dest_addr = (transfer.destination + i) & 0b0001_1111_1111_1111;
				match src_addr {
					// Cartage address
					0x0000..0x8000 | 0xA000..0xC000 => {
						if let Some(cartridge) = cartridge {
							vram[dest_addr as usize] = cartridge.read(src_addr)
						}
					}

					0xC000..0xD000 => {
						vram[dest_addr as usize] = wram.get_bank(0)[(src_addr - 0xC000) as usize]
					}

					0xD000..0xE000 => {
						vram[dest_addr as usize] =
							wram.get_bank(wram.get_bank_number())[(src_addr - 0xD000) as usize]
					}
					_ => unreachable!("DMA outside range"),
				}
			}
		}

		self.transfer = None;
	}

	pub fn step(&mut self) {
		let transfer_done = if let Some(transfer) = &mut self.transfer {
			transfer.bytes_sent += 1;
			transfer.bytes_sent >= transfer.total_length
		} else {
			true
		};

		if transfer_done {
			self.transfer = None;
		}
	}
}
