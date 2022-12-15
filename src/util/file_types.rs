use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Entry {
	File(String, String),
	Dir(String, HashMap<String, Entry>),
}
