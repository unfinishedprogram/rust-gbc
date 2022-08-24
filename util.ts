export function stringToUint8Arr(str:string) {
	const buffer = new ArrayBuffer(str.length*2); // 2 bytes for each char
  const bufView = new Uint16Array(buffer);

	const size = str.length;
  for (let i = 0; i < size; i++) {
    bufView[i] = str.charCodeAt(i);
	}

  return buffer;
}