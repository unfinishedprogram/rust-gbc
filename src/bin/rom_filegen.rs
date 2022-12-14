use clap::Parser;

use std::{collections::HashMap, fs::read_dir, io::Error};

#[derive(Parser, Debug)]
struct Args {
	folder: String,
}

#[derive(Debug)]
enum Entry {
	File,
	Dir(HashMap<String, Entry>),
}

fn recursive_dir_parse(root: String) -> Result<Entry, Error> {
	let folder = read_dir(root)?;

	let mut dir_hash: HashMap<String, Entry> = HashMap::new();

	for file in folder.flatten() {
		if file.metadata()?.is_dir() {
			dir_hash.insert(
				file.file_name().into_string().unwrap(),
				recursive_dir_parse(file.path().to_str().unwrap().to_owned())?,
			);
		} else {
			dir_hash.insert(file.file_name().into_string().unwrap(), Entry::File);
		}
	}
	Ok(Entry::Dir(dir_hash))
}

fn main() -> Result<(), Error> {
	let args = Args::parse();

	let entries = recursive_dir_parse(args.folder).unwrap();

	println!("{entries:#?}");

	Ok(())
}
