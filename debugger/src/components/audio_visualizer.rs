use std::collections::VecDeque;

use egui::{Color32, Pos2, Sense, Stroke, Ui, Vec2};
use gameboy::Gameboy;

pub struct AudioVisualizer {
	sample_width: usize,
	samples: VecDeque<f32>,
}

impl Default for AudioVisualizer {
	fn default() -> Self {
		Self::new()
	}
}

impl AudioVisualizer {
	pub fn new() -> Self {
		let width = 1024 * 8;

		AudioVisualizer {
			sample_width: width,
			samples: VecDeque::from(vec![0.0; width]),
		}
	}

	pub fn draw(&mut self, gameboy: &mut Gameboy, ui: &mut Ui) {
		let samples_to_pull = gameboy.audio.buffered_samples();
		let samples = gameboy.audio.pull_samples(samples_to_pull);

		for (l, r) in samples {
			self.samples.pop_back();
			self.samples.push_front((l + r) / 2.0);
		}

		ui.label("Audio Visualizer");

		let (response, painter) =
			ui.allocate_painter(Vec2::new(ui.available_width(), 300.0), Sense::hover());

		let painter_rect = response.rect;

		let horizontal_scale = painter_rect.width() / self.samples.len() as f32;
		let stroke = Stroke::new(2.0, Color32::from_rgb(128, 128, 255));

		let lines =
			self.samples
				.clone()
				.into_iter()
				.enumerate()
				.map_windows(|[(idx_a, a), (idx_b, b)]| {
					[
						painter_rect.left_center().to_vec2()
							+ Vec2::new(
								*idx_a as f32 * horizontal_scale,
								*a * painter_rect.height(),
							),
						painter_rect.left_center().to_vec2()
							+ Vec2::new(
								*idx_b as f32 * horizontal_scale,
								*b * painter_rect.height(),
							),
					]
				});

		lines.for_each(|[a, b]| {
			painter.line_segment([a.to_pos2(), b.to_pos2()], stroke);
		});
	}
}
