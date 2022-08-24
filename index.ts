import { load_rom_and_run } from "./pkg/gbc_emu";
import tetris_rom from "./roms/tetris.gb?raw";
import { stringToUint8Arr } from "./util";

var log = console.log;

const rom_data = stringToUint8Arr(tetris_rom);

console.log(load_rom_and_run(new Uint8Array(rom_data)));
