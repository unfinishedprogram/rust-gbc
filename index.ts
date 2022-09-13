import { create_cpu, load_rom_and_run, step_cpu, cpu_info } from "./pkg/gbc_emu";
import { readRomData } from "./util";
import { ICpuData, Display }  from "./display";

const display = new Display();

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

let cpu = create_cpu();
let rom = await readRomData("tetris.gb");
let boot = await readRomData("dmg_boot.bin", true);

let i = 0;

load_rom_and_run(cpu, rom, boot);
let run;
run = () => {
	try {
		step_cpu(cpu);
		poll();
	} catch (e) {
		clearInterval(i);
		throw e;
	}
}

const poll = () => {
	let data:ICpuData = JSON.parse(cpu_info(cpu));
	display.update(data);
}

// document.addEventListener("click", run);

i = setInterval(run, 25);



document.body.appendChild(display.elm);
