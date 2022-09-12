import { load_rom_and_run } from "./pkg/gbc_emu";
import { readRomData } from "./util";

// var log = console.log;

(window as any).log = (str:string) => {
	try {
		let arr = str.split(" ");
		arr[0] = "("
		arr.push(")")
		let obj = eval(arr.join(" "));
		console.log(obj);
	} catch {
		console.log(str);
	}

}
async function play() {
	let rom = await readRomData("tetris.gb");
	let boot = await readRomData("dmg_boot.bin", true);
	load_rom_and_run(rom, boot);
}

play();