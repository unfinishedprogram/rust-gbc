use std::ops::{Bound, Range};

use eframe::wgpu::Label;
use egui::Color32;
use gameboy::gb_sm83::memory_mapper::MemoryMapper;

pub struct AddressInfo {
    label: Option<String>,
}

#[derive(Debug)]
pub enum IORegister {
    DIV,
    TIMA,
    TMA,
    TAC,
    NR10,
    NR11,
    NR12,
    NR14,
    NR21,
    NR22,
    NR24,
    NR30,
    NR31,
    NR32,
    NR33,
    NR41,
    NR42,
    NR43,
    NR44,
    NR50,
    NR51,
    NR52,
    LCDC,
    STAT,
    SCY,
    SCX,
    LY,
    LYC,
    DMA,
    BGP,
    OBP0,
    OBP1,
    WY,
    WX,
    SB,
    SC,
    IF,
    IE,
}

#[derive(Debug)]
pub enum AddressRange {
    RomBank0,
    RomBankN,
    VRam,
    ExternalRam,
    WRamBank0,
    WRamBankN,
    Mirror,
    SpriteAttributes,
    Unusable,
    IORegisters(Option<IORegister>),
    HighRam,
    InterruptEnable,
}

fn map_range(addr: u16) -> AddressRange {
    use AddressRange::*;
    use IORegister::*;

    match addr {
        0x0000..0x4000 => RomBank0,
        0x4000..0x8000 => RomBankN,
        0x8000..0xA000 => VRam,
        0xA000..0xC000 => ExternalRam,
        0xC000..0xD000 => WRamBank0,
        0xD000..0xE000 => WRamBankN,
        0xE000..0xFE00 => Mirror,
        0xFE00..0xFEA0 => SpriteAttributes,
        0xFEA0..0xFF00 => Unusable,
        0xFF04 => IORegisters(Some(DIV)),
        0xFF05 => IORegisters(Some(TIMA)),
        0xFF06 => IORegisters(Some(TMA)),
        0xFF07 => IORegisters(Some(TAC)),
        0xFF10 => IORegisters(Some(NR10)),
        0xFF11 => IORegisters(Some(NR11)),
        0xFF12 => IORegisters(Some(NR12)),
        0xFF14 => IORegisters(Some(NR14)),
        0xFF16 => IORegisters(Some(NR21)),
        0xFF17 => IORegisters(Some(NR22)),
        0xFF19 => IORegisters(Some(NR24)),
        0xFF1A => IORegisters(Some(NR30)),
        0xFF1B => IORegisters(Some(NR31)),
        0xFF1C => IORegisters(Some(NR32)),
        0xFF1E => IORegisters(Some(NR33)),
        0xFF20 => IORegisters(Some(NR41)),
        0xFF21 => IORegisters(Some(NR42)),
        0xFF22 => IORegisters(Some(NR43)),
        0xFF23 => IORegisters(Some(NR44)),
        0xFF24 => IORegisters(Some(NR50)),
        0xFF25 => IORegisters(Some(NR51)),
        0xFF26 => IORegisters(Some(NR52)),
        0xFF40 => IORegisters(Some(LCDC)),
        0xFF41 => IORegisters(Some(STAT)),
        0xFF42 => IORegisters(Some(SCY)),
        0xFF43 => IORegisters(Some(SCX)),
        0xFF44 => IORegisters(Some(LY)),
        0xFF45 => IORegisters(Some(LYC)),
        0xFF46 => IORegisters(Some(DMA)),
        0xFF47 => IORegisters(Some(BGP)),
        0xFF48 => IORegisters(Some(OBP0)),
        0xFF49 => IORegisters(Some(OBP1)),
        0xFF4A => IORegisters(Some(WY)),
        0xFF4B => IORegisters(Some(WX)),
        0xFF01 => IORegisters(Some(SB)),
        0xFF02 => IORegisters(Some(SC)),
        0xFF0F => IORegisters(Some(IF)),
        0xFF00..0xFF80 => IORegisters(None),
        0xFF80..0xFFFE => HighRam,
        _ => InterruptEnable,
    }
}

pub fn get_addr_info(addr: u16) -> (AddressRange, Option<&'static str>) {
    let range = map_range(addr);
    (range, None)
}
