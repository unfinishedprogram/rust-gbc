use egui::Ui;
use gameboy::debugger::{Breakpoint, DEBUGGER};

use super::breakpoint_selector::BreakpointSelector;

#[derive(Default)]
pub struct BreakpointManager {
    add_breakpoint_open: Option<AddBreakpointMenu>,
}

#[derive(Default)]
pub struct AddBreakpointMenu {
    breakpoint: Option<Breakpoint>,
}

impl BreakpointManager {
    pub fn draw(&mut self, ui: &mut Ui) {
        {
            DEBUGGER.lock().unwrap().breakpoints.retain(|point| {
                ui.horizontal(|ui| {
                    ui.label(format!("{:?}", point));
                    !ui.button("🗙").clicked()
                })
                .inner
            });
        }

        if ui.button("Add Breakpoint").clicked() {
            self.add_breakpoint_open = Some(AddBreakpointMenu::default());
        }

        if self.add_breakpoint_open.is_some() {
            if let Some(menu) = self.add_breakpoint_open.as_mut() {
                let breakpoint_selector = BreakpointSelector::new(&mut menu.breakpoint);
                ui.add(breakpoint_selector);
            }

            if ui.button("Add").clicked() {
                if let Some(AddBreakpointMenu {
                    breakpoint: Some(breakpoint),
                }) = self.add_breakpoint_open.take()
                {
                    let mut debugger = DEBUGGER.lock().unwrap();
                    debugger.add_breakpoint(breakpoint);
                }
            }
        }
    }
}
