use std::{
	collections::HashMap,
	fs::{self, read_dir},
	io::Error,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Entry {
	File(String, String),
	Dir(String, HashMap<String, Entry>),
}

fn recursive_dir_parse(root: String) -> Result<Entry, Error> {
	let folder = read_dir(&root)?;

	let mut dir_hash: HashMap<String, Entry> = HashMap::new();

	for file in folder.flatten() {
		if file.metadata()?.is_dir() {
			let name = file.file_name().into_string().unwrap();
			let entries = recursive_dir_parse(file.path().to_str().unwrap().to_owned())?;
			dir_hash.insert(name, entries);
		} else {
			let name = file.file_name().into_string().unwrap();
			let path = file.path().to_string_lossy().into_owned();
			dir_hash.insert(name.clone(), Entry::File(name, path));
		}
	}
	Ok(Entry::Dir(root, dir_hash))
}

fn main() -> Result<(), Error> {
	let entries = recursive_dir_parse("roms".to_string()).unwrap();
	let serialized = serde_json::to_string(&entries).unwrap();
	fs::write("roms.json", serialized.as_bytes()).unwrap();
	println!("cargo:rerun-if-changed=roms");
	Ok(())
}
