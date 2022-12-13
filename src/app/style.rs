use eframe::epaint::Shadow;
use egui::{style::Widgets, Color32, Rounding, Stroke, Style, Visuals};

pub fn apply(ctx: &egui::Context) {
	let mut visuals = Visuals::default();
	let mut widgets = Widgets::default();
	let mut style = Style::default();

	style.spacing.item_spacing = (5.0, 5.0).into();
	style.spacing.button_padding = (10.0, 5.0).into();

	widgets.noninteractive.bg_fill = Color32::from_rgb(0x1c, 0x21, 0x2b);
	widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(0xBB, 0xBB, 0xBB));

	widgets.inactive.rounding = Rounding::default().at_least(2.0);

	visuals.widgets = widgets;
	visuals.window_rounding = Rounding::default().at_least(2.0);
	visuals.window_shadow = Shadow::small_dark();
	visuals.override_text_color = Some(Color32::from_rgb(0xc5, 0xc5, 0xc5));
	visuals.hyperlink_color = Color32::from_rgb(0x00, 0x96, 0xcf);

	ctx.set_style(style);
	ctx.set_visuals(visuals);
}
