import * as wasm from "/gbc-emu.js"


async function get_available_roms() {
    const res = await fetch("./roms.json");
    const json = await res.json();
    const dirs = json["Dir"][1];

    const games = dirs.filter(dir => dir["Dir"][0] == ("roms/games"))[0]["Dir"][1];

    console.log(games);
    return games.map(game => {
        let name = game.File[0];
        let path = game.File[1];
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