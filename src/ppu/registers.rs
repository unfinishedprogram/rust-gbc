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
