use crate::util::color;
use eframe::epaint::Shadow;
use egui::{style::Widgets, Rounding, Stroke, Style, Visuals};

pub fn apply(ctx: &egui::Context) {
	let mut visuals = Visuals::default();
	let mut widgets = Widgets::default();
	let mut style = Style::default();

	style.spacing.item_spacing = (5.0, 5.0).into();
	style.spacing.button_padding = (10.0, 5.0).into();

	widgets.noninteractive.bg_fill = color(0x1c212b);
	widgets.noninteractive.bg_stroke = Stroke::new(1.0, color(0xBBBBBB));

	widgets.inactive.rounding = Rounding::default().at_least(2.0);

	visuals.widgets = widgets;
	visuals.window_rounding = Rounding::default().at_least(2.0);
	visuals.window_shadow = Shadow::small_dark();
	visuals.override_text_color = Some(color(0xc5c5c5));
	visuals.hyperlink_color = color(0x0096cf);

	ctx.set_style(style);
	ctx.set_visuals(visuals);
}
