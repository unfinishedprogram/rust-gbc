use serde::{Deserialize, Serialize};
use sm83::memory_mapper::MemoryMapper;

use crate::Gameboy;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct OamDmaState {
	start_delay: u32,
	dma_cycle: u16,
	dma_active: bool,
	dma_addr: u16,
}

pub fn step_oam_dma(gb: &mut Gameboy) {
	if gb.oam_dma.start_delay > 0 {
		gb.oam_dma.start_delay = gb.oam_dma.start_delay.saturating_sub(1);
		if gb.oam_dma.start_delay == 0 {
			gb.oam_dma.dma_active = true;
			gb.oam_dma.dma_cycle = 0;
		}
	} else if gb.oam_dma.dma_active && gb.oam_dma.dma_cycle < 160 {
		let val = gb.read(gb.oam_dma.dma_addr + gb.oam_dma.dma_cycle);
		gb.ppu.oam[gb.oam_dma.dma_cycle as usize] = val;
		gb.oam_dma.dma_cycle += 1;

		if gb.oam_dma.dma_cycle == 160 {
			gb.oam_dma.dma_active = false;
		}
	}
}

impl OamDmaState {
	pub fn start_oam_dma(&mut self, value: u8) {
		let value = if value > 0xDF { value - 0x20 } else { value };
		let real_addr = (value as u16) << 8;

		log::warn!("OAM DMA request from {:04X}", real_addr);
		self.dma_addr = real_addr;
		self.start_delay = 2;
	}

	pub fn oam_is_accessible(&self) -> bool {
		!self.dma_active
	}
}
