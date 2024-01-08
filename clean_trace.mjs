import {readFileSync, writeFileSync} from "node:fs";

const FILE_IN = "./logs/WarioLandCorrect.log";
const FILE_OUT = "./logs/WarioLandCorrectClean.log";

const LINE_COUNT = 1000;

let regex = new RegExp(":  .. .. ..  ", "g");

let text = readFileSync(FILE_IN).toString();
console.log("Loaded file")
let out = text.replace(regex, " ");
console.log("Applied replacement")

out = out.split("\n").slice(0,LINE_COUNT).join("\n")
console.log("Applied line reduction")

writeFileSync(FILE_OUT, out);
console.log("Saving file")
