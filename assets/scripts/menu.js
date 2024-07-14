import * as wasm from "/application.js"
import { configure_keybindings } from "./controller_input.js";

console.log(wasm);

const main_element = document.querySelector("#main");
const menu_toggle = document.querySelector("#toggle_menu");

const menu = {
    open: false,
    element: document.querySelector("#menu"),
    animate_open_element: document.querySelector("#animate_open"),
    animate_close_element: document.querySelector("#animate_close"),
    toggle_open: function () {
        if (this.open) {
            main_element.removeAttribute("data-menu-open")
            this.animate_close_element.beginElement();
        } else {
            main_element.setAttribute("data-menu-open", "")
            this.animate_open_element.beginElement();
        }
        this.open = !this.open;
    }
}
const menu_content = document.querySelector("#menu_content");

let save = document.createElement("button");
save.innerText = "Save";
save.onclick = () => wasm.save_save_state(0);
menu_content.appendChild(save);

let load = document.createElement("button");
load.innerText = "Load";
load.onclick = () => wasm.load_save_state(0);
menu_content.appendChild(load);

let toggle_vsync = document.createElement("input");
toggle_vsync.type = "checkbox";
toggle_vsync.id = "toggle_vsync";
toggle_vsync.checked = false;
toggle_vsync.onchange = () => wasm.set_vsync(toggle_vsync.checked);

let toggle_vsync_label = document.createElement("label");
toggle_vsync_label.htmlFor = "toggle_vsync";
toggle_vsync_label.innerText = "VSync";
menu_content.appendChild(toggle_vsync_label);
menu_content.appendChild(toggle_vsync);

let edit_keybindings = document.createElement("button");
edit_keybindings.innerText = "Edit Keybindings";
edit_keybindings.onclick = () => configure_keybindings();
menu_content.appendChild(edit_keybindings);

menu_toggle.addEventListener("click", () => {
    menu.toggle_open();
})