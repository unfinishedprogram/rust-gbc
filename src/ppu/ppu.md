# PPU

## OAM (Object Attribute Map)

### Byte 0

 Y-Position - 16

### Byte 1

 X-Position - 8

### Byte 2

 Tile Number:
  0x8000 + Value (U8)

### Byte 3

 Flags:
Bit 7    OBJ-to-BG Priority
          0 = Sprite is always rendered above background
          1 = Background colors 1-3 overlay sprite, sprite is still rendered above color 0
Bit 6    Y-Flip
          If set to 1 the sprite is flipped vertically, otherwise rendered as normal
Bit 5    X-Flip
          If set to 1 the sprite is flipped horizontally, otherwise rendered as normal
Bit 4    Palette Number
          If set to 0, the OBP0 register is used as the palette, otherwise OBP1
Bit 3-0  CGB-Only flags
