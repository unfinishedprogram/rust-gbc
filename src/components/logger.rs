use super::drawable::{Drawable, DrawableMut};
use egui::{Color32, Ui};
use egui_extras::{Size, TableBuilder};

pub struct Logger {
	logs: Vec<LogMessage>,
	warn_enabled: bool,
	error_enabled: bool,
	info_enabled: bool,
	debug_enabled: bool,
}

pub enum LogMessageType {
	Error,
	Warn,
	Info,
	Debug,
}

pub type LogMessage = (LogMessageType, String);

impl Drawable for LogMessage {
	fn draw(&self, ui: &mut Ui) {
		use LogMessageType::*;
		let (msg, color, icon) = match self {
			(Error, msg) => (msg, Color32::from_rgb(255, 102, 102), "⛔"),
			(Info, msg) => (msg, Color32::from_rgb(110, 156, 247), "ℹ"),
			(Warn, msg) => (msg, Color32::from_rgb(255, 179, 102), "⚠"),
			(Debug, msg) => (msg, Color32::from_rgb(255, 102, 255), "💡"),
		};

		ui.label(
			egui::RichText::new(format!("{} {}", icon, msg))
				.strong()
				.color(color)
				.monospace()
				.size(16.0),
		);
	}
}

impl DrawableMut for Logger {
	fn draw(&mut self, ui: &mut Ui) {
		ui.heading("Logs");

		ui.collapsing("Levels", |ui| {
			ui.checkbox(&mut self.error_enabled, "⛔ Error");
			ui.checkbox(&mut self.warn_enabled, "⚠ Warn");
			ui.checkbox(&mut self.info_enabled, "ℹ Info");
			ui.checkbox(&mut self.debug_enabled, "💡 Debug");
		});

		ui.separator();

		TableBuilder::new(ui)
			.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
			.striped(true)
			.scroll(true)
			.stick_to_bottom(true)
			.column(Size::Remainder {
				range: (0.0, 500.0),
			})
			.body(|body| {
				let mut iter = self.logs.iter().filter(|item| match item {
					(LogMessageType::Error, _) => self.error_enabled,
					(LogMessageType::Warn, _) => self.warn_enabled,
					(LogMessageType::Info, _) => self.info_enabled,
					(LogMessageType::Debug, _) => self.debug_enabled,
				});

				let count = iter.clone().count();

				body.rows(18.0, count, |_, mut row| {
					if let Some(item) = iter.next() {
						row.col(|ui| item.draw(ui));
					}
				});
			});
	}
}

impl Logger {
	pub fn info<S: Into<String>>(&mut self, msg: S) {
		self.log((LogMessageType::Info, msg.into()));
	}
	pub fn error<S: Into<String>>(&mut self, msg: S) {
		self.log((LogMessageType::Error, msg.into()));
	}
	pub fn warn<S: Into<String>>(&mut self, msg: S) {
		self.log((LogMessageType::Warn, msg.into()));
	}
	pub fn debug<S: Into<String>>(&mut self, msg: S) {
		self.log((LogMessageType::Debug, msg.into()));
	}

	fn log(&mut self, msg: LogMessage) {
		self.logs.push(msg);
		if self.logs.len() > 200 {
			self.logs.remove(0);
		}
	}
}

impl Default for Logger {
	fn default() -> Self {
		let mut logger = Logger {
			logs: vec![],
			warn_enabled: true,
			error_enabled: true,
			info_enabled: false,
			debug_enabled: false,
		};

		logger.error("Test Error");
		logger.warn("Test Warn");
		logger.info("Test Info");
		logger.debug("Test Debug");
		logger
	}
}
