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

### Blarggs ✅

#### Instructions

|     | Test                          |
| --- | ----------------------------- |
| ✅   | roms/01-special.gb            |
| ✅   | roms/02-interrupts.gb         |
| ✅   | roms/03-op sp,hl.gb           |
| ✅   | roms/04-op r,imm.gb           |
| ✅   | roms/05-op rp.gb              |
| ✅   | roms/06-ld r,r.gb             |
| ✅   | roms/07-jr,jp,call,ret,rst.gb |
| ✅   | roms/08-misc instrs.gb        |
| ✅   | roms/09-op r,r.gb             |
| ✅   | roms/10-bit ops.gb            |
| ✅   | roms/11-op a,(hl).gb          |


### 02/04/2023 
test result: FAILED. 1171 passed; 1757 failed; 0 ignored; 0 measured; 0 filtered out; finished in 20.42s
test result: FAILED. 1179 passed; 1748 failed; 0 ignored; 0 measured; 0 filtered out; finished in 20.21s
test result: FAILED. 1191 passed; 1736 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.01s