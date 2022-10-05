use crate::app::{drawable::DrawableMut, managed_input::ManagedInput};
use egui::{ScrollArea, Style};
use std::collections::HashMap;

pub struct BreakpointManager {
	breakpoints: HashMap<u16, bool>,
	address_input: ManagedInput<u16>,
}

impl BreakpointManager {
	pub fn add_breakpoint(&mut self, addr: u16) {
		self.breakpoints.insert(addr, true);
	}

	pub fn remove_breakpoint(&mut self, addr: u16) {
		self.breakpoints.remove(&addr);
	}

	pub fn break_on(&self, addr: u16) -> bool {
		return match self.breakpoints.get(&addr) {
			Some(true) => true,
			_ => false,
		};
	}
}

fn validate_address(string: &str) -> bool {
	match (u16::from_str_radix(string, 16), string.len()) {
		(_, 0) | (Result::Ok(_), 1..=4) => true,
		(_, _) => false,
	}
}

impl Default for BreakpointManager {
	fn default() -> Self {
		Self {
			breakpoints: HashMap::new(),
			address_input: ManagedInput::<u16>::new(validate_address, |string| {
				u16::from_str_radix(string, 16).ok()
			}),
		}
	}
}

impl DrawableMut for BreakpointManager {
	fn draw(&mut self, ui: &mut egui::Ui) {
		ui.vertical(|ui| {
			ui.heading("Breakpoints");
			ui.horizontal(|ui| {
				self.address_input.draw(ui);
				let mut style = Style::default();
				style.spacing.button_padding = (2.0, 2.0).into();
				ui.ctx().set_style(style);
				if ui.button("add").clicked() {
					if let Some(address) = self.address_input.get_value() {
						self.add_breakpoint(address);
					}
					self.address_input.clear();
				}
			});

			ScrollArea::vertical()
				.always_show_scroll(true)
				.max_height(200.0)
				.show(ui, |ui| {
					ui.set_min_height(200.0);
					ui.vertical(|ui| {
						ui.separator();
						let keys: Vec<u16> = self
							.breakpoints
							.keys()
							.into_iter()
							.map(|val| val.to_owned())
							.collect();

						for addr in keys {
							ui.horizontal_top(|ui| {
								ui.style_mut().spacing.button_padding = (2.0, 2.0).into();

								if ui.button("❌").clicked() {
									self.remove_breakpoint(addr);
								}

								ui.label(format!("{:04X}", addr));
							});
						}
					});
				})
		});
	}
}
