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

### 02/16/2023 
test result: FAILED. 1178 passed; 1748 failed; 0 ignored; 0 measured; 0 filtered out; finished in 19.76s
test result: FAILED. 1183 passed; 1743 failed; 0 ignored; 0 measured; 0 filtered out; finished in 19.84s
test result: FAILED. 1188 passed; 1738 failed; 0 ignored; 0 measured; 0 filtered out; finished in 35.76s
test result: FAILED. 1192 passed; 1734 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.64s

### 02/20/2023 
test result: FAILED. 1199 passed; 1727 failed; 0 ignored; 0 measured; 0 filtered out; finished in 30.66s
<!-- Modified ppu timing -->
test result: FAILED. 1200 passed; 1726 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.02s
<!-- Modified ppu memory access -->
test result: FAILED. 1230 passed; 1696 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.96s
<!-- Proper Stat IRQ blocking -->
test result: FAILED. 1294 passed; 1632 failed; 0 ignored; 0 measured; 0 filtered out; finished in 30.62s
<!-- Undocumented registers -->
test result: FAILED. 1311 passed; 1615 failed; 0 ignored; 0 measured; 0 filtered out; finished in 32.16s


test result: FAILED. 1291 passed; 1601 failed; 0 ignored; 0 measured; 0 filtered out; finished in 27.48s
test result: FAILED. 1292 passed; 1600 failed; 0 ignored; 0 measured; 0 filtered out; finished in 27.48s

test result: FAILED. 1289 passed; 1595 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.14s