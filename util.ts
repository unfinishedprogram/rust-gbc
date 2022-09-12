export async function readRomData(rom:string, print=false) {
  let res = await fetch(rom);
  let arr = new Uint8Array(await res.arrayBuffer());
  return arr;
}