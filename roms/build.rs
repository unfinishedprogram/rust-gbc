use std::{
	cmp::Ordering,
	fs::{self, DirEntry, read_dir},
	io::Error,
};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Entry {
	File { path: String },
	Dir { path: String, entries: Vec<Entry> },
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
			let path = file.path().to_string_lossy().into_owned();
			dir_listing.push(Entry::File { path });
		}
	}
	Ok(Entry::Dir {
		path: root.into(),
		entries: dir_listing,
	})
}

fn main() -> Result<(), Error> {
	let serialized = get_roms_serialized()?;
	let out_dir = std::env::var("OUT_DIR").unwrap();
	let out_path = std::path::Path::new(&out_dir).join("roms.json");

	println!("cargo:rustc-env=GENERATED_JSON_PATH={}", out_path.display());

	println!("cargo:rerun-if-changed=roms");
	println!("cargo:rerun-if-changed=build.rs");
	fs::write(out_path, serialized.as_bytes())?;
	Ok(())
}

pub fn get_roms_serialized() -> Result<String, Error> {
	let entries = recursive_dir_parse("roms")?;
	let res = serde_json::to_string(&entries)?;
	Ok(res)
}
