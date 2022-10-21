use egui::{Ui, WidgetText};

pub trait Drawable {
	fn draw(&self, ui: &mut Ui);
	fn draw_window(&self, ui: &mut Ui, title: impl Into<WidgetText>) {
		egui::Window::new(title)
			.resizable(false)
			.show(ui.ctx(), |ui| self.draw(ui));
	}
}

pub trait DrawableMut {
	fn draw(&mut self, ui: &mut Ui);
	fn draw_window(&mut self, ui: &mut Ui, title: impl Into<WidgetText>) {
		egui::Window::new(title)
			.resizable(false)
			.show(ui.ctx(), |ui| self.draw(ui));
	}
}
