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

load_rom_and_run(cpu, rom, boot);

document.addEventListener("click", () => {
	step_cpu(cpu);
	let data:ICpuData = JSON.parse(cpu_info(cpu));
	display.update(data);
})

document.body.appendChild(display.elm);
