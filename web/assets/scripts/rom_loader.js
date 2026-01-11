import app from "./main_app.js";

async function get_available_roms() {
  const res = await fetch("./roms.json");
  const json = await res.json();
  const dirs = json.dir.entries;
  console.log(json);
  return dirs;
}

function make_rom_button(rom) {
  if ("dir" in rom) {
    let dropdown = document.createElement("details");
    let summary_elm = document.createElement("summary");
    summary_elm.textContent = rom.dir.path;
    dropdown.appendChild(summary_elm);

    rom.dir.entries.forEach((entry) => {
      dropdown.appendChild(make_rom_button(entry));
    });
    return dropdown;
  } else if ("file" in rom) {
    const elm = document.createElement("button");
    let path = rom.file.path;
    let name = path.split("/").pop().split(".")[0];
    elm.onclick = () => load_rom({ name, path });
    elm.innerText = name;
    return elm;
  }

  const elm = document.createElement("button");
  elm.onclick = () => load_rom(rom);
  elm.innerText = rom.name;
  return elm;
}

async function load_rom({ name, path }) {
  window.history.pushState("rust-gbc", `${name}`, `?rom=${path}`);
  await load_rom_internal({ name, path });
}

async function load_rom_internal({ name, path }) {
  let rom_data = new Uint8Array(await (await fetch(path)).arrayBuffer());
  app.load_rom(rom_data, path);
}

get_available_roms().then((roms) => {
  document
    .getElementById("roms_container")
    .append(...roms.map(make_rom_button));
});

function load_rom_if_in_query_params() {
  const urlParams = new URLSearchParams(window.location.search);
  const rom = urlParams.get("rom");
  if (rom) {
    load_rom_internal({ name: rom, path: rom });
  }
}

load_rom_if_in_query_params();
