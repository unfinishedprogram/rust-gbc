import { load_rom_and_run } from "./pkg/gbc_emu";
import { readRomData } from "./util";

// var log = console.log;

(window as any).log = (str:string) => {
	let arr = str.split(" ");
	arr[0] = "("
	arr.push(")")
	let obj = eval(arr.join(" "));
	console.log(obj);
}

// const rom_data = stringToUint8Arr(tetris_rom);
// console.log(rom_data);
readRomData("tetris.gb").then(rom => {
	load_rom_and_run(rom);
});

// console.log(load_rom_and_run(new Uint8Array(rom_data)));
