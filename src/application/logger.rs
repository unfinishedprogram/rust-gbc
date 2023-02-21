use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use log;
use log::{Level, Metadata, Record};
use wasm_bindgen::prelude::wasm_bindgen;
pub struct GBLogger;

struct GBLoggerState {
	log_count: usize,
	log_back: usize,
	max_logs: usize,
	logs: VecDeque<String>,
}

impl GBLoggerState {
	pub fn new(buffer_size: usize) -> Self {
		Self {
			log_back: 0,
			log_count: 0,
			max_logs: buffer_size,
			logs: VecDeque::with_capacity(buffer_size),
		}
	}
}

lazy_static! {
	static ref LOGS: Arc<Mutex<GBLoggerState>> =
		Arc::new(Mutex::new(GBLoggerState::new(2048 * 512)));
}
pub static LOGGER: GBLogger = GBLogger;

impl log::Log for GBLogger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() <= Level::Info
	}

	fn log(&self, record: &Record) {
		// if self.enabled(record.metadata()) {
		if let Ok(state) = &mut LOGS.lock() {
			state.log_count += 1;
			if state.logs.len() + 1 >= state.max_logs {
				state.log_back += 1;
				state.logs.pop_back();
			}

			state
				.logs
				.push_front(format!("{} - {}", record.level(), record.args()));
		}
		// }
	}

	fn flush(&self) {}
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn get_logs(from: usize, to: usize) -> String {
	let Ok(logs) = LOGS.lock() else {return "".to_string()};

	let [from, to] = [from - logs.log_back, to - logs.log_back];

	let logs: Vec<String> = logs
		.logs
		.range(from..to)
		.map(|val| val.to_string())
		.collect();

	serde_json::to_string::<Vec<String>>(&logs).unwrap()
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn log_count() -> usize {
	let Ok(logs) = LOGS.lock() else {return 0};
	logs.log_count
}
