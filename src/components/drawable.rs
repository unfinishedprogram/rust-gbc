use egui::Ui;

pub trait Drawable {
	fn draw(&self, ui: &mut Ui);
}

pub trait DrawableMut {
	fn draw(&mut self, ui: &mut Ui);
}
