use super::super::managed_input::ManagedInput;
use crate::app::drawable::DrawableMut;
use egui_extras::{Size, TableBuilder};
use std::collections::HashMap;

pub struct BreakpointManager {
	enabled: bool,
	breakpoints: HashMap<u16, bool>,
	address_input: ManagedInput<u16>,
}

impl BreakpointManager {
	pub fn enable(&mut self) {
		self.enabled = true;
	}

	pub fn disable(&mut self) {
		self.enabled = false;
	}

	pub fn enable_breakpoint(&mut self, addr: u16) {
		if let Some(state) = self.breakpoints.get_mut(&addr) {
			*state = true;
		} else {
			self.breakpoints.insert(addr, true);
		}
	}

	pub fn disable_breakpoint(&mut self, addr: u16) {
		if let Some(state) = self.breakpoints.get_mut(&addr) {
			*state = false;
		}
	}

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
			enabled: false,
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

				if ui.button("add").clicked() {
					if let Some(address) = self.address_input.get_value() {
						self.add_breakpoint(address);
					}
					self.address_input.clear();
				}
			});

			ui.vertical(|ui| {
				TableBuilder::new(ui)
					.scroll(true)
					.striped(true)
					.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
					.stick_to_bottom(true)
					.column(Size::Remainder {
						range: (0.0, 500.0),
					});
			});

			ui.vertical(|ui| {
				let keys: Vec<u16> = self
					.breakpoints
					.keys()
					.into_iter()
					.map(|val| val.to_owned())
					.collect();

				for addr in keys {
					ui.horizontal_top(|ui| {
						if ui
							.checkbox(
								self.breakpoints.get_mut(&addr).unwrap_or(&mut false),
								format!("{:04X}", addr),
							)
							.clicked()
						{}

						if ui.button("❌").clicked() {
							self.remove_breakpoint(addr);
						}
					});
				}
			});
		});
	}
}
