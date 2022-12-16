use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Entry {
	File(String, String),
	Dir(String, Vec<Entry>),
}
