# wasm compatible game boy emulator written in rust

## Long term goals

- Game boy color support
- All Blarggs tests passing
- Full debugger
  - Breakpoints
  - Decompiler
  - Live instruction view
- Faster than realtime performance
- Android/IOS application
- Configurable controls

## Opcode Surface Check

|    | x0 | x1 | x2 | x3 | x4 | x5 | x6 | x7 | x8 | x9 | xA | xB | xC | xD | xE | xF |
|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|
| 0x | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 1x | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 2x | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 3x | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 4x | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 5x | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 6x | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 7x | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 8x | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| 9x | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| Ax | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| Bx | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| Cx | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| Dx | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| Ex | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |
| Fx | ✔️ |    |    |    |    |    |    |    |    |    |    |    |    |    |    |    |

    > Legend
    > ✔️ Working
    > ❌ Not working
