// 40 sprites maximum
pub enum PPURegister {
	LCDC = 0xFF40,
	STAT = 0xFF41,
	SCX = 0xFF43,
	SCY = 0xFF42,
	LY = 0xFF44,
	LYC = 0xFF45,
	WY = 0xFF4A,
	WX = 0xFF4B,

	VramStart = 0x8000,
	VramEnd = 0x97FF,

	OamStart = 0xFE00,
	OamEnd = 0xFE9F,

	BG1Start = 0x9800,
	BG1End = 0x9BFF,

	BG2Start = 0x9C00,
	BG2End = 0x9FFF,
}

// Bit 6 - LYC=LY Coincidence Interrupt (1=Enable)
// Bit 5 - Mode 2 OAM Interrupt         (1=Enable)
// Bit 4 - Mode 1 V-Blank Interrupt     (1=Enable)
// Bit 3 - Mode 0 H-Blank Interrupt     (1=Enable)
// Bit 2 - Coincidence Flag  (0:LYC<>LY, 1:LYC=LY)
// Bit 1-0 - Mode Flag       (Mode 0-3, see below)
