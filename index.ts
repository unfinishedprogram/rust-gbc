import { create_cpu, get_current_cpu_state, load_rom_and_run, step_cpu } from "./pkg/gbc_emu";
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
	let cpu = create_cpu();
	let rom = await readRomData("tetris.gb");
	let boot = await readRomData("dmg_boot.bin", true);
	load_rom_and_run(cpu, rom, boot);
	for(let i = 0; i < 256; i++) {
		step_cpu(cpu);
	}
}

play();