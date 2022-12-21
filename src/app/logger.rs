use std::sync::Mutex;

use log::{Level, Metadata, Record};

pub struct Logger {
	pub logs: Mutex<Vec<String>>,
}

impl Logger {
	pub fn new() -> Self {
		Self {
			logs: Mutex::new(vec![]),
		}
	}
}

impl Default for Logger {
	fn default() -> Self {
		Self::new()
	}
}

impl log::Log for Logger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() <= Level::Info
	}

	fn log(&self, record: &Record) {
		if let Ok(mut logs) = self.logs.lock() {
			logs.push(format!("{} - {}", record.level(), record.args()))
		}
	}

	fn flush(&self) {
		if let Ok(mut logs) = self.logs.lock() {
			logs.clear();
		}
	}
}
