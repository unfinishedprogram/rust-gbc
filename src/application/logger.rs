use wasm_bindgen::prelude::wasm_bindgen;
pub struct GBLogger;
use gameboy::{debugger, Debugger};

pub static LOGGER: Debugger = Debugger;

#[allow(dead_code)]
#[wasm_bindgen]
pub fn get_logs(from: usize, to: usize) -> String {
	let logs: Vec<String> = debugger::get_range(from, to)
		.into_iter()
		.map(|val| format!("{val:?}"))
		.collect();

	serde_json::to_string::<Vec<String>>(&logs).unwrap()
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn log_count() -> usize {
	gameboy::debugger::log_count()
}
