use serde::{Deserialize, Serialize};

use crate::ppu::PPU;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct OamDmaState {
	oam_accessible: bool,
	cycles_remaining: u32,
	start_delay: u32,
	dma_request: Option<Vec<u8>>,
}

const OAM_DMA_DURATION: u32 = 159;

impl OamDmaState {
	pub fn step(&mut self, m_cycles: u32, ppu: &mut PPU) {
		if self.start_delay != 0 {
			self.start_delay = self.start_delay.saturating_sub(m_cycles);
		}

		if self.cycles_remaining == 0 {
			self.oam_accessible = true;
		} else {
			self.cycles_remaining = self.cycles_remaining.saturating_sub(m_cycles);
		}

		if self.start_delay == 0 {
			if let Some(data) = self.dma_request.take() {
				for (i, data) in data.iter().enumerate() {
					ppu.oam[i] = *data;
				}
				self.cycles_remaining = OAM_DMA_DURATION;
				self.oam_accessible = false;
			}
		}
	}

	pub fn start_oam_dma(&mut self, data: Vec<u8>) {
		self.dma_request = Some(data);
		self.start_delay = 2;
	}

	pub fn oam_is_accessible(&self) -> bool {
		self.oam_accessible
	}
}
