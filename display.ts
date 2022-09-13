export interface ICpuData {
	memory:number[],
	registers: {
		bytes:number[],
		sp:number, 
		pc:number
	}
}


export class Display {
	public readonly elm:HTMLDivElement;
	private static registerNames = "ABCDEFHL";

	constructor() {
		this.elm = document.createElement("div");
	}

	public update(info:ICpuData):void {
		this.elm.innerHTML = "";
		for(let i = 0; i < 8; i++) {
			let e = document.createElement("div");
			e.innerText = `${Display.registerNames.split("")[i]}:0x${info.registers.bytes[i].toString(16).padStart(2,"0")}`
			this.elm.appendChild(e);
		}

		let sp = document.createElement("div");
		let pc = document.createElement("div");

		sp.innerText = `SP:0x${info.registers.sp.toString(16).padStart(4,"0")}`;
		pc.innerText = `PC:0x${info.registers.pc.toString(16).padStart(4,"0")}`;

		this.elm.appendChild(sp);
		this.elm.appendChild(pc);
	}
}