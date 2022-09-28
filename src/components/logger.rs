use super::drawable::Drawable;
use egui::{Color32, Ui};
use egui_extras::{Size, TableBuilder};

pub struct Logger {
	logs: Vec<LogMessage>,
}

pub enum LogMessage {
	Error(String),
	Warn(String),
	Info(String),
	Debug(String),
}

impl Drawable for LogMessage {
	fn draw(&self, ui: &mut Ui) {
		let (msg, color, icon) = match self {
			LogMessage::Error(msg) => (msg, Color32::from_rgb(255, 102, 102), "⛔"),
			LogMessage::Info(msg) => (msg, Color32::from_rgb(110, 156, 247), "ℹ"),
			LogMessage::Warn(msg) => (msg, Color32::from_rgb(255, 179, 102), "⚠"),
			LogMessage::Debug(msg) => (msg, Color32::from_rgb(255, 102, 255), "💡"),
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

impl Drawable for Logger {
	fn draw(&self, ui: &mut Ui) {
		ui.heading("Logs");
		ui.separator();
		ui.spacing_mut().item_spacing = (0.0, 0.0).into();

		TableBuilder::new(ui)
			.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
			.striped(true)
			.scroll(true)
			.stick_to_bottom(true)
			.column(Size::Remainder {
				range: (0.0, 500.0),
			})
			.body(|body| {
				body.rows(18.0, self.logs.len(), |row_index, mut row| {
					row.col(|ui| self.logs[row_index].draw(ui));
				});
			});
	}
}

impl Logger {
	pub fn info<S: Into<String>>(&mut self, msg: S) {
		self.log(LogMessage::Info(msg.into()));
	}
	pub fn error<S: Into<String>>(&mut self, msg: S) {
		self.log(LogMessage::Error(msg.into()));
	}
	pub fn warn<S: Into<String>>(&mut self, msg: S) {
		self.log(LogMessage::Warn(msg.into()));
	}
	pub fn debug<S: Into<String>>(&mut self, msg: S) {
		self.log(LogMessage::Debug(msg.into()));
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
		let mut logger = Logger { logs: vec![] };
		logger.error("Test Error");
		logger.warn("Test Warn");
		logger.info("Test Info");
		logger.debug("Test Debug");
		logger
	}
}
