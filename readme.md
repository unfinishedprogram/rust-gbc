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


## Test Status

### Blarggs

|     | Test                          |
| --- | ----------------------------- |
| ✅   | roms/01-special.gb            |
| ❌   | roms/02-interrupts.gb         |
| ✅   | roms/03-op sp,hl.gb           |
| ❌   | roms/04-op r,imm.gb           |
| ✅   | roms/05-op rp.gb              |
| ✅   | roms/06-ld r,r.gb             |
| ✅   | roms/07-jr,jp,call,ret,rst.gb |
| ✅   | roms/08-misc instrs.gb        |
| ❌   | roms/09-op r,r.gb             |
| ❌   | roms/10-bit ops.gb            |
| ❌   | roms/11-op a,(hl).gb          |