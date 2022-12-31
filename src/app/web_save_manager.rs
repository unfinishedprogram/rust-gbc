use web_sys::window;

use super::save_manager::{SaveError, SaveManager, SaveState};

pub struct WebSaveManager {}

impl SaveManager for WebSaveManager {
	fn load_save_states() -> Result<Vec<SaveState>, SaveError> {
		use SaveError::*;
		let data = window()
			.ok_or(NoSource)?
			.local_storage()
			.or(Err(NoSource))?
			.ok_or(NoSource)?
			.get_item("saves")
			.or(Err(NoSource))?
			.ok_or(NoSource)?;

		serde_json::from_str::<Vec<SaveState>>(&data).or(Err(Deserialization))
	}

	fn save_save_state(state: SaveState) -> Result<(), SaveError> {
		use SaveError::*;

		let mut states = WebSaveManager::load_save_states().unwrap_or(vec![]);
		states.push(state);

		let storage = window()
			.ok_or(NoSource)?
			.local_storage()
			.or(Err(NoSource))?
			.ok_or(NoSource)?;

		let serialized_data =
			serde_json::to_string::<Vec<SaveState>>(&states).or(Err(Serialization))?;

		storage
			.set_item("saves", &serialized_data)
			.or(Err(Deserialization))?;

		Ok(())
	}
}
