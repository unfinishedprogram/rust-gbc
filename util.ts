export async function readRomData(rom:string) {
  let res = await fetch(rom);
  let arr = await res.arrayBuffer();
  return new Uint8Array(arr);
}