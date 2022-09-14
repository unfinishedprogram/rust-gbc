use std::{future::Future, process::Output};

use crate::{cpu::Cpu, components::{register_view::register_view, memory_view::memory_view}};
use rfd::AsyncFileDialog;
use egui_extras::{Size, TableBuilder};
use poll_promise::Promise;

use crate::components;

enum RomLoadType {
    Bios(Promise<ehttp::Result<Vec<u8>>>),
    Rom(Promise<ehttp::Result<Vec<u8>>>)
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(serde::Deserialize, serde::Serialize)]
pub struct EmulatorManager {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    cpu:Cpu,

    #[serde(skip)]
    loaded_file_data:Option<RomLoadType>
}

impl Default for EmulatorManager {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            cpu:Cpu::new(),
            loaded_file_data:None::<RomLoadType>,
        }
    }
}

impl EmulatorManager {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		Default::default()
	}
}

impl eframe::App for EmulatorManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, value , cpu, loaded_file_data } = self;

        match &self.loaded_file_data {
            Some(RomLoadType::Bios(rom)) => match rom.ready() {
                Some(rom) => {
                    self.cpu.load_boot_rom(rom.as_ref().unwrap().into_iter().as_slice());
                    self.loaded_file_data = None;
                },
                None => {},
            }
            Some(RomLoadType::Rom(rom)) => match rom.ready() {
                Some(rom) => {
                    self.cpu.load_cartridge(rom.as_ref().unwrap().into_iter().as_slice());
                    self.loaded_file_data = None;
                },
                None => {},
            }
            None => {},
        }

        register_view(ctx, &self.cpu);
        memory_view(ctx, &self.cpu);

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Write Stuff");
                ui.text_edit_singleline(label);
            });

            ui.label(format!("PC:{:x}", self.cpu.registers.pc));
            ui.label(format!("SP:{:x}", self.cpu.registers.sp));

            if ui.button("step").clicked() {
                self.cpu.execute_next_instruction();
            }

            if ui.button("Load Bios").clicked() {
                let promise = (self.loaded_file_data.get_or_insert_with(|| {
                    let ctx = ctx.clone();
                    let (sender, promise) = Promise::new();
                    let request = ehttp::Request::get("dmg_boot.bin");
                    ehttp::fetch(request, move |response| {
                        let data = response.and_then(parse_response);
                        sender.send(data); // send the results back to the UI thread.
                        ctx.request_repaint(); // wake up UI thread
                    });

                    RomLoadType::Bios(promise)
                }));
            }

            if ui.button("Load Rom").clicked() {
                let promise = (self.loaded_file_data.get_or_insert_with(|| {
                    let ctx = ctx.clone();
                    let (sender, promise) = Promise::new();
                    let request = ehttp::Request::get("tetris.gb");
                    ehttp::fetch(request, move |response| {
                        let data = response.and_then(parse_response);
                        sender.send(data); // send the results back to the UI thread.
                        ctx.request_repaint(); // wake up UI thread
                    });

                    RomLoadType::Rom(promise) 
                }));
            }            

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });
    }
}


fn parse_response(response: ehttp::Response) -> Result<Vec<u8>, String> {
    Ok(response.bytes)
}