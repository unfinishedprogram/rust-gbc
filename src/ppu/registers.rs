// 40 sprites maximum
pub enum PPURegister {
	LCDC = 0xFF40,
	STAT = 0xFF41,
	WY = 0xFF4A,
	WX = 0xFF4B,
	FCY = 0xFF42,
	VramStart = 0x8000,
	VramEnd = 0x97FF,
	OamStart = 0xFE00,
	OamEnd = 0xFE9F,
	// 9800-$9BFF //  BG MAPS
	// $9C00-$9FFF
}
