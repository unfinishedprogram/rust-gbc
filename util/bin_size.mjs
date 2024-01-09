import { execSync } from 'child_process';
import { readFileSync } from 'fs';

execSync("trunk build --release application.html");
execSync("wasm-objdump --section=Code -d ./dist/application_bg.wasm > wasm_sizes.txt");
const dump = readFileSync('./wasm_sizes.txt', 'utf8');
execSync("rm ./wasm_sizes.txt");

const is_func_start = (line) => {
  return line.includes("func[");
}

function parse_dump(dump) {
  let lines = dump.split("\n");
  // Count lines in each function

  let func_lines = {};
  let current_func = null;
  for (let line of lines) {
    if (is_func_start(line)) {
      current_func = line;
      func_lines[current_func] = 0;
    } else if (current_func) {
      func_lines[current_func] += 1;
    }
  }
  return func_lines;
}



console.log(
  Object.entries(parse_dump(dump)).sort(([_, a], [__, b]) => (b - a))
);