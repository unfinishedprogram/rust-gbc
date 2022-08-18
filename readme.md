# wasm compatible game boy emulator written in rust

## Opcodes

|    | x0 | x1 | x2 | x3 | x4 | x5 | x6 | x7 | x8 | x9 | xA | xB | xC | xD | xE | xF |
|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|
| 0x |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 1x |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 2x |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 3x |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 4x |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 5x |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 6x |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 7x |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 8x |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 9x |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| Ax |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| Bx |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| Cx |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| Dx |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| Ex |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| Fx |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |

    > Legend
    > ✔️ Working
    > ❌ Not working

## Memory

| Start | End  | Description                     | Notes                                                       |
|-------|------|---------------------------------|-------------------------------------------------------------|
| 0000  | 3FFF | 16KB ROM bank 00                | From cartridge, usually a fixed bank                        |
| 4000  | 7FFF | 16KB ROM Bank 01~NN             | From cartridge, switchable bank via MBC (if any)            |
| 8000  | 9FFF | 8KB Video RAM (VRAM)            | Only bank 0 in Non-CGB mode Switchable bank 0/1 in CGB mode |
| A000  | BFFF | 8KB External RAM                | In cartridge, switchable bank if any                        |
| C000  | CFFF | 4KB Work RAM (WRAM) bank 0      |                                                             |
| D000  | DFFF | 4KB Work RAM (WRAM) bank 1~N    | Only bank 1 in Non-CGB modeSwitchable bank 1~7 in CGB mode  |
| E000  | FDFF | Mirror of C000~DDFF (ECHO RAM)  | Typically not used                                          |
| FE00  | FE9F | Sprite attribute table (OAM)    |                                                             |
| FEA0  | FEFF | Not Usable                      |                                                             |
| FF00  | FF7F | I/O Registers                   |                                                             |
| FF80  | FFFE | High RAM (HRAM)                 |                                                             |
| FFFF  | FFFF | Interrupts Enable Register (IE) |                                                             |
