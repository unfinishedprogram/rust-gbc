// 40 sprites maximum

pub enum PPURegister {
	WY = 0xFF4A,
	WX = 0xFF4B,
	VRAM_START = 0x8000,
	VRAM_END = 0x97FF,

	// 9800-$9BFF //  BG MAPS
	// $9C00-$9FFF

	// $FE00-$FE9F // OAM
	LCDC,
}

// OAM 4 bytes
// [y, x, TileNumber, SpriteFlag];

// Sprite Flag
// Bit 7    OBJ-to-BG Priority
//           0 = Sprite is always rendered above background
//           1 = Background colors 1-3 overlay sprite, sprite is still rendered above color 0
// Bit 6    Y-Flip
//           If set to 1 the sprite is flipped vertically, otherwise rendered as normal
// Bit 5    X-Flip
//           If set to 1 the sprite is flipped horizontally, otherwise rendered as normal
// Bit 4    Palette Number
//           If set to 0, the OBP0 register is used as the palette, otherwise OBP1
// Bit 3-0  CGB-Only flags
