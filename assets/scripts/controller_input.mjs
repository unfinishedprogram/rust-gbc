const keybinding_dialog = document.querySelector("#keybinding_dialog");

function load_keybindings() {
    let bindings = JSON.parse(localStorage.getItem("keybindings"));

    if (bindings) {
        return bindings;
    } else {
        let defaults = {
            a: "z",
            b: "x",
            select: "tab",
            start: "enter",
            right: "arrowright",
            left: "arrowleft",
            up: "arrowup",
            down: "arrowdown",
        };

        store_keybindings(defaults);
    }
    localStorage.getItem("keybindings");
}

function store_keybindings(keybindings) {
    localStorage.setItem("keybindings", JSON.stringify(keybindings));
}

let current_keybindings = load_keybindings();

window.controller_state_raw = {
    a: false,
    b: false,
    select: false,
    start: false,
    right: false,
    left: false,
    up: false,
    down: false,
};
window.controller_state = JSON.stringify(window.controller_state_raw);

export function button_up(button) {
    if (button in window.controller_state_raw) {
        window.controller_state_raw[button] = false;
        window.controller_state = JSON.stringify(window.controller_state_raw);
    } else {
        console.warn(`Unknown button ${button}`);
    }
}

export function button_down(button) {
    if (button in window.controller_state_raw) {
        window.controller_state_raw[button] = true;
        window.controller_state = JSON.stringify(window.controller_state_raw);
    } else {
        console.warn(`Unknown button ${button}`);
    }
}

export function clear_joypad_state() {
    for (let button in window.controller_state_raw) {
        window.controller_state_raw[button] = false;
    }
    window.controller_state = JSON.stringify(window.controller_state_raw);
}

export function configure_keybindings() {
    const new_keybindings = {};

    for (let button of Object.keys(current_keybindings)) {
        new_keybindings[button] = current_keybindings[button];
    }

    console.log(new_keybindings)


    clear_joypad_state();

    let current_key_listener = null;

    // Open the dialog
    keybinding_dialog.showModal();

    let button_fields = ["a", "b", "select", "start", "right", "left", "up", "down"]
        .map(button_id => ({
            button_id,
            button: document.querySelector(`#${button_id}`),
            current: document.querySelector(`.${button_id} > .current`),
        }));

    const set_disabled_all = (value) => {
        button_fields.forEach(({ button }) => button.disabled = value);
    }

    button_fields.forEach(({ button_id, button, current }) => {
        current.innerText = new_keybindings[button_id];

        button.addEventListener("click", (e) => {
            e.preventDefault();
            current.innerText = "Press a key";

            const on_keydown = (event) => {
                event.preventDefault();
                if (event.key == "Escape") {
                    current.innerText = new_keybindings[button_id];
                } else {
                    current.innerText = event.key.toLowerCase();
                    new_keybindings[button_id] = event.key.toLowerCase();
                }
                set_disabled_all(false);
                document.removeEventListener("keydown", on_keydown);
            }

            current_key_listener = on_keydown;

            set_disabled_all(true);
            document.addEventListener("keydown", on_keydown);
        });
    })

    const form = document.querySelector("#keybinding_dialog > form");

    const onsubmit = (e) => {
        console.log("Updating Keybindings");
        current_keybindings = new_keybindings;
    }

    keybinding_dialog.onclose = () => {
        form.removeEventListener("submit", onsubmit);
        document.removeEventListener("keydown", current_key_listener);
    };

    form.addEventListener("submit", onsubmit);
    document.querySelector("#keybinding_dialog > form").addEventListener("submit", onsubmit);
}

function initialize_input() {
    const update_button_state = (event, state) => {
        let fn = state ? button_down : button_up;

        console.log(current_keybindings);
        console.log(window.controller_state);

        for (let button of Object.keys(current_keybindings)) {
            if (current_keybindings[button] == event.key.toLowerCase()) {
                fn(button)
                return;
            }
        }
    }

    addEventListener("keydown", (event) => update_button_state(event, true));
    addEventListener("keyup", (event) => update_button_state(event, false));
}

// setInterval(() => console.log(current_keybindings), 100)

initialize_input();