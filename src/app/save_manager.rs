use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::emulator::{cartridge::memory_bank_controller::Cartridge, EmulatorState};
use chrono::NaiveDateTime;
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveState {
	pub date: NaiveDateTime,
	pub game_title: String,
	pub data: String,
}

#[derive(Debug, Clone)]
pub enum SaveError {
	Serialization,
	Deserialization,
	InvalidGame,
	NoSource,
}

pub trait SaveManager {
	fn load_save_states() -> Result<Vec<SaveState>, SaveError>;
	fn save_save_state(state: SaveState) -> Result<(), SaveError>;
}

impl TryFrom<&EmulatorState> for SaveState {
	type Error = SaveError;

	fn try_from(value: &EmulatorState) -> Result<Self, SaveError> {
		let data = serde_json::to_string(value).or(Err(SaveError::Serialization))?;

		let Cartridge(_, _, info) = &value
			.cartridge_state
			.as_ref()
			.ok_or(SaveError::InvalidGame)?;

		let game_title = info.title.clone();
		let date = chrono::offset::Utc::now().naive_utc();

		Ok(Self {
			date,
			game_title,
			data,
		})
	}
}

impl Display for SaveState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}] : {}", self.date, self.game_title)
	}
}
