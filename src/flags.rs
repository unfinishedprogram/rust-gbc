pub type BitFlagRef = (u16, u8);

pub enum BitFlag {
	// Interrupt flags
	InterruptVBlank = (0xFF0F, 0),
	InterruptLcdStat = (0xFF0F, 1),
	InterruptTimer = (0xFF0F, 2),
	InterruptSerial = (0xFF0F, 3),
	InterruptJoyPad = (0xFF0F, 4),

	// LCD Control 0xFF40
	BGDisplay = (0xFF40, 0),
	OBJDisplayEnable = (0xFF40, 1),
	OBJSize = (0xFF40, 2),
	BGAndWindowTileDataSelect = (0xFF40, 4),
	BGTileMapDisplaySelect = (0xFF40, 3),
	WindowDisplayEnable = (0xFF40, 5),
	WindowTileMapDisplaySelect = (0xFF40, 6),
	LcdDisplayEnable = (0xFF40, 7),

	// Timer control
	TimerStop = (0xFF07, 2),
}
