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
	hdma5: u8,
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
		if let Some(_) = &mut self.transfer {
			if value & BIT_7 != 0 {
				self.transfer = None;
				self.hdma5 |= BIT_7;
			}
			return;
		}

		let mode = if value & BIT_7 == BIT_7 {
			TransferMode::HBlank
		} else {
			TransferMode::GeneralPurpose
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

	pub fn update_length(&mut self) {
		if let Some(transfer) = &self.transfer {
			let length = transfer.total_length - transfer.bytes_sent;
			let length = (length / 0x10 - 1) as u8;
			self.hdma5 = length;
		}
	}

	pub fn read_length(&self) -> u8 {
		return self.hdma5;
	}

	pub fn step_single(
		&mut self,
		ppu: &mut PPU,
		cartridge: &mut Option<Cartridge>,
		wram: &mut impl BankedWorkRam,
		v_ram_bank: &VRAMBank,
	) {
		for _ in 0..0x10 {
			if let Some(transfer) = &mut self.transfer {
				let vram = match v_ram_bank {
					VRAMBank::Bank0 => &mut ppu.v_ram_bank_0,
					VRAMBank::Bank1 => &mut ppu.v_ram_bank_1,
				};

				let i = transfer.bytes_sent;
				let src_addr = (transfer.source + i) as usize;
				let dest_addr = (transfer.destination + i) as usize;

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

				transfer.bytes_sent += 1;
				if transfer.bytes_sent == transfer.total_length {
					self.transfer = None;
					return;
				}
			}
		}
		self.update_length();
	}

	pub fn step_controller(
		&mut self,
		ppu: &mut PPU,
		cartridge: &mut Option<Cartridge>,
		wram: &mut impl BankedWorkRam,
		v_ram_bank: &VRAMBank,
	) {
		let Some(transfer) = &self.transfer else { return };

		match transfer.mode {
			TransferMode::GeneralPurpose => {
				while self.transfer.is_some() {
					self.step_single(ppu, cartridge, wram, v_ram_bank)
				}
				self.hdma5 = 0xFF;
			}
			TransferMode::HBlank => {
				self.step_single(ppu, cartridge, wram, v_ram_bank);
				self.hdma5 |= BIT_7;
			}
		}
	}
}
