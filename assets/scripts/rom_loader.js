import * as wasm from "/application.js"


async function get_available_roms() {
    const res = await fetch("./roms.json");
    const json = await res.json();
    const dirs = json.dir.entries;
    const games = dirs.filter(({ dir }) => dir.path == "roms/games")[0].dir.entries;

    console.log(games);
    return games.map(game => {
        let name = game.file.path.split("/").pop().replace(".gbc", "").replace(".gb", "");
        let path = game.file.path;
        return { name, path };
    })
}

function make_rom_button(rom) {
    const elm = document.createElement("button");
    elm.onclick = () => load_rom(rom);
    elm.innerText = rom.name;
    return elm;
}

async function load_rom({ name, path }) {
    let rom_data = new Uint8Array(await (await fetch(path)).arrayBuffer());
    wasm.load_rom(rom_data, `{"LocalUrl": "${path}"}`);
}

get_available_roms().then(roms => {
    document.getElementById("roms_container").append(
        ...roms.map(make_rom_button)
    )
})

function load_rom_if_in_query_params() {
    const urlParams = new URLSearchParams(window.location.search);
    const rom = urlParams.get('rom');
    if (rom) {
        load_rom({ name: rom, path: rom });
    }
}

load_rom_if_in_query_params();