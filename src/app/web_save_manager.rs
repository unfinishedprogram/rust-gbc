use serde::Deserialize;
use web_sys::{window, Storage};

use super::save_manager::{SaveError, SaveManager, SaveState};

pub struct WebSaveManager {}

impl WebSaveManager {
	fn get_item<T>(key: &str) -> Result<T, SaveError>
	where
		T: for<'a> Deserialize<'a>,
	{
		let storage = WebSaveManager::get_storage()?;

		let Ok(Some(item)) = storage.get_item(key) else {
			return Err(SaveError::NoSource)
		};

		let Ok(item) = serde_json::from_str::<T>(&item) else {
			return Err(SaveError::Deserialization)
		};

		Ok(item)
	}

	fn get_storage() -> Result<Storage, SaveError> {
		use SaveError::*;

		let Some(window) = window() else {
			return Err(NoSource)
		};

		let Ok(Some(storage)) = window.local_storage() else {
			return Err(NoSource)
		};

		Ok(storage)
	}
}

impl SaveManager for WebSaveManager {
	fn load_save_state(slot: usize) -> Result<SaveState, SaveError> {
		use SaveError::*;

		let data = window()
			.ok_or(NoSource)?
			.local_storage()
			.or(Err(NoSource))?
			.ok_or(NoSource)?
			.get_item(&slot.to_string())
			.or(Err(NoSource))?
			.ok_or(NoSource)?;

		serde_json::from_str::<SaveState>(&data).or(Err(Deserialization))
	}

	fn get_save_states() -> Vec<Option<String>> {
		if let Ok(index) = WebSaveManager::get_item("index") {
			index
		} else {
			(0..10).map(|_| None).collect()
		}
	}

	fn save_save_state(state: SaveState, slot: usize) -> Result<(), SaveError> {
		use SaveError::*;
		if slot >= 10 {
			return Err(IndexOutOfBounds(slot));
		}
		let storage = WebSaveManager::get_storage()?;

		let mut current_index = WebSaveManager::get_save_states();

		current_index[slot] = Some(state.info.to_string());

		let serialized_data = serde_json::to_string::<SaveState>(&state).or(Err(Serialization))?;
		let serialized_index =
			serde_json::to_string::<Vec<Option<String>>>(&current_index).or(Err(Serialization))?;

		storage
			.set_item(&slot.to_string(), &serialized_data)
			.or(Err(Deserialization))?;

		storage
			.set_item("index", &serialized_index)
			.or(Err(Deserialization))?;

		Ok(())
	}
}
