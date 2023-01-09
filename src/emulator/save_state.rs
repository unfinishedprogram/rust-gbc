use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::emulator::{cartridge::memory_bank_controller::Cartridge, EmulatorState};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RomSource {
	ExternalUrl(String),
	LocalUrl(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveState {
	pub data: String,
	pub info: SaveStateEntry,
	pub rom_source: Option<RomSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveStateEntry {
	pub date: NaiveDateTime,
	pub game_title: String,
}

#[derive(Debug, Clone)]
pub enum SaveError {
	Serialization,
	Deserialization,
	InvalidGame,
	NoSource,
	MissingIndex,
	IndexOutOfBounds(usize),
}

pub trait SaveManager {
	fn load_save_state(slot: usize) -> Result<SaveState, SaveError>;
	fn save_save_state(state: SaveState, slot: usize) -> Result<(), SaveError>;
	fn get_save_states() -> Vec<Option<String>>;
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
			rom_source: info.rom_source.clone(),
			info: SaveStateEntry { date, game_title },
			data,
		})
	}
}

impl Display for SaveStateEntry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"[{}] : {}",
			self.date.format("%Y-%m-%d %H:%M"),
			self.game_title
		)
	}
}
