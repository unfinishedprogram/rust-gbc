import * as wasm from "/gbc-emu.js"
let speed_index = 3;

const speed_options = [
    ["1/8x", 0.125],
    ["1/4x", 0.25],
    ["1/2x", 0.5],
    ["1x", 1.0],
    ["2x", 2.0],
    ["4x", 4.0],
    ["8x", 8.0],
];

const speed_label = document.createElement("span");
const elm = document.createElement("div");
const up = document.createElement("button");
const down = document.createElement("button");


const update_speed_display = () => {
    speed_label.textContent = speed_options[speed_index][0];
}

const speed_up = () => {
    if (speed_index < speed_options.length - 1) {
        speed_index++;
        update_speed_display();
        wasm.set_speed(speed_options[speed_index][1]);
    }
}

const speed_down = () => {
    if (speed_index > 0) {
        speed_index--;
        update_speed_display();
        wasm.set_speed(speed_options[speed_index][1]);
    }
}

speed_label.textContent = "1x"
up.textContent = "x2"
down.textContent = "/2"

up.onclick = speed_up;
down.onclick = speed_down;


elm.append(down, speed_label, up);

const menu_content = document.querySelector("#menu_content");
menu_content.appendChild(elm);
