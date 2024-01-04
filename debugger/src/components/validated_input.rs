use egui::Color32;

pub struct ValidatedInput<T> {
	label: Option<String>,
	text: String,
	content: Option<T>,
	validate: fn(&str) -> Result<T, String>,
}

impl<T> ValidatedInput<T> {
	pub fn default(mut self, default: impl Into<String>) -> Self {
		self.text = default.into();
		self.content = Some((self.validate)(&self.text).expect("invalid default value"));
		self
	}

	pub fn label(mut self, label: impl Into<String>) -> Self {
		self.label = Some(label.into());
		self
	}

	pub fn new(validate: fn(&str) -> Result<T, String>) -> Self {
		Self {
			text: String::new(),
			label: None,
			content: None,
			validate,
		}
	}

	pub fn value(&self) -> &Option<T> {
		&self.content
	}
}

impl<T> egui::Widget for &mut ValidatedInput<T> {
	fn ui(self, ui: &mut egui::Ui) -> egui::Response {
		ui.horizontal(|ui| {
			if let Some(label) = self.label.as_ref() {
				ui.label(label);
			}
			let res = ui.text_edit_singleline(&mut self.text);

			match (self.validate)(&self.text) {
				Ok(v) => _ = self.content.insert(v),
				Err(err) => _ = ui.colored_label(Color32::RED, err),
			};

			res
		})
		.inner
	}
}
