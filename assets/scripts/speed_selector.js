import * as wasm from "/application.js"

let speed_index = 3;

const speed_options = [
    ["1/8x", 1 / 8],
    ["1/4x", 1 / 4],
    ["1/2x", 1 / 2],
    ["1x", 1],
    ["2x", 2],
    ["4x", 4],
    ["8x", 8],
    ["16x", 16],
    ["32x", 32],
    ["64x", 64],
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
