use egui::Ui;

pub trait Drawable {
	fn draw(&self, ui: &mut Ui);
}
