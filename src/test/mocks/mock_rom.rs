pub fn create_rom(program_data: Vec<u8>) -> Vec<u8> {
	let mut data = vec![0; 0x200];
	for (index, byte) in program_data.into_iter().enumerate() {
		data[index + 0x100] = byte;
	}

	data
}
