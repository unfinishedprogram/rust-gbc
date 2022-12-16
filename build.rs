use std::{
	cmp::Ordering,
	fs::{self, read_dir, DirEntry},
	io::Error,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Entry {
	File(String, String),
	Dir(String, Vec<Entry>),
}

fn compare_dir_entry(a: &DirEntry, b: &DirEntry) -> Ordering {
	a.file_name()
		.to_ascii_lowercase()
		.cmp(&b.file_name().to_ascii_lowercase())
}

// Recursively parses a given directory structure into a nested enum
fn recursive_dir_parse(root: &str) -> Result<Entry, Error> {
	let folder = read_dir(root)?;

	let mut dir_listing: Vec<Entry> = vec![];
	let mut files = folder.flatten().collect::<Vec<DirEntry>>();
	files.sort_by(compare_dir_entry);

	for ref file in files {
		if file.metadata()?.is_dir() {
			let entries = recursive_dir_parse(file.path().to_str().unwrap())?;
			dir_listing.push(entries);
		} else {
			let name = file.file_name().into_string().unwrap();
			let path = file.path().to_string_lossy().into_owned();
			dir_listing.push(Entry::File(name, path));
		}
	}
	Ok(Entry::Dir(root.into(), dir_listing))
}

fn main() -> Result<(), Error> {
	let entries = recursive_dir_parse("roms")?;
	let serialized = serde_json::to_string(&entries)?;
	fs::write("roms.json", serialized.as_bytes())?;
	println!("cargo:rerun-if-changed=roms");

	Ok(())
}
