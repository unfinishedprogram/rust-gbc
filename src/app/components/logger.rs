use crate::app::drawable::{Drawable, DrawableMut};
use egui::{Color32, RichText, Ui};
use egui_extras::{Size, TableBuilder};
use std::fmt::Display;

pub static mut INSTANCE: Logger = Logger::new();

pub struct Logger {
	logs: Vec<LogMessage>,
	warn_enabled: bool,
	error_enabled: bool,
	info_enabled: bool,
	debug_enabled: bool,
}

#[derive(Debug)]
pub enum LogMessageType {
	Error,
	Warn,
	Info,
	Debug,
}

impl Display for LogMessageType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {:?}", self.icon(), self)
	}
}

impl LogMessageType {
	pub fn icon(&self) -> &str {
		match self {
			LogMessageType::Error => "⛔",
			LogMessageType::Warn => "⚠",
			LogMessageType::Info => "ℹ",
			LogMessageType::Debug => "💡",
		}
	}
	pub fn color(&self) -> Color32 {
		match self {
			LogMessageType::Error => Color32::from_rgb(255, 102, 102),
			LogMessageType::Warn => Color32::from_rgb(255, 179, 102),
			LogMessageType::Info => Color32::from_rgb(110, 156, 247),
			LogMessageType::Debug => Color32::from_rgb(255, 102, 255),
		}
	}
}

pub type LogMessage = (LogMessageType, String);

impl Drawable for LogMessage {
	fn draw(&self, ui: &mut Ui) {
		let (log_type, msg) = self;
		ui.label(
			egui::RichText::new(format!("{} {}", log_type.icon(), msg))
				.strong()
				.color(log_type.color())
				.monospace()
				.size(16.0),
		);
	}
}

pub unsafe fn draw(ui: &mut Ui) {
	ui.heading("Logs");
	use LogMessageType::*;
	ui.collapsing("Levels", |ui| {
		ui.checkbox(
			&mut INSTANCE.error_enabled,
			RichText::new(format!("{}", Error)).color(Error.color()),
		);
		ui.checkbox(
			&mut INSTANCE.warn_enabled,
			RichText::new(format!("{}", Warn)).color(Warn.color()),
		);
		ui.checkbox(
			&mut INSTANCE.info_enabled,
			RichText::new(format!("{}", Info)).color(Info.color()),
		);
		ui.checkbox(
			&mut INSTANCE.debug_enabled,
			RichText::new(format!("{}", Debug)).color(Debug.color()),
		);
	});

	ui.separator();
	TableBuilder::new(ui)
		.scroll(true)
		.striped(true)
		.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
		.stick_to_bottom(true)
		.column(Size::Remainder {
			range: (0.0, 500.0),
		})
		.body(|body| {
			let filtered: Vec<&LogMessage> = INSTANCE
				.logs
				.iter()
				.filter(|item| match item {
					(LogMessageType::Error, _) => INSTANCE.error_enabled,
					(LogMessageType::Warn, _) => INSTANCE.warn_enabled,
					(LogMessageType::Info, _) => INSTANCE.info_enabled,
					(LogMessageType::Debug, _) => INSTANCE.debug_enabled,
				})
				.collect();

			let count = filtered.len();

			body.rows(18.0, count, |index, mut row| {
				row.col(|ui| filtered[index].draw(ui));
			});
		});
}

pub fn info<S: Into<String>>(msg: S) {
	unsafe {
		INSTANCE.info(msg);
	}
}
pub fn error<S: Into<String>>(msg: S) {
	unsafe {
		INSTANCE.error(msg);
	}
}
pub fn warn<S: Into<String>>(msg: S) {
	unsafe {
		INSTANCE.warn(msg);
	}
}
pub fn debug<S: Into<String>>(msg: S) {
	unsafe {
		INSTANCE.debug(msg);
	}
}

impl Logger {
	pub const fn new() -> Logger {
		let logger = Logger {
			logs: vec![],
			warn_enabled: true,
			error_enabled: true,
			info_enabled: true,
			debug_enabled: true,
		};
		logger
	}

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
