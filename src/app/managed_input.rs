use super::drawable::DrawableMut;

pub struct ManagedInput<T> {
	input_value: String,
	validate: fn(&str) -> bool,
	parse: fn(&str) -> Option<T>,
}

impl<T> DrawableMut for ManagedInput<T> {
	fn draw(&mut self, ui: &mut egui::Ui) {
		let last_value = self.input_value.clone();
		if ui.text_edit_singleline(&mut self.input_value).changed() {
			if !(self.validate)(&self.input_value) {
				self.input_value = last_value.clone();
			}
		}
	}
}

impl<T> ManagedInput<T> {
	pub fn new(validate: fn(&str) -> bool, parse: fn(&str) -> Option<T>) -> Self {
		Self {
			input_value: String::from(""),
			validate,
			parse,
		}
	}

	pub fn get_value(&self) -> Option<T> {
		(self.parse)(&self.input_value)
	}

	pub fn clear(&mut self) {
		self.input_value.clear();
	}
}
