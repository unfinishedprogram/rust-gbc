use egui::Ui;
use gameboy::Gameboy;
use poll_promise::Promise;

#[derive(Default)]
pub struct RomLoader {
    url: String,
    promise: Option<Promise<ehttp::Result<RomResource>>>,
    error_msg: Option<String>,
}

pub struct RomResource {
    response: ehttp::Response,
}

impl RomLoader {
    pub fn draw(&mut self, ui: &mut Ui, gameboy: &mut Gameboy) {
        ui.text_edit_singleline(&mut self.url);

        if ui.button("load").clicked() {
            self.error_msg = None;
            let ctx = ui.ctx().clone();

            let (sender, promise) = Promise::new();

            let request = ehttp::Request::get(&self.url);

            ehttp::fetch(request, move |response| {
                ctx.request_repaint();
                let resource = response.map(|response| RomResource { response });
                sender.send(resource);
            });

            self.promise = Some(promise);
        }

        if let Some(promise) = &self.promise {
            if let Some(result) = promise.ready() {
                match result {
                    Ok(resource) => {
                        *gameboy = Gameboy::cgb();
                        gameboy.load_rom(&resource.response.bytes, None);
                    }
                    Err(error) => {
                        let msg = if error.is_empty() { "Error" } else { error };
                        self.error_msg = Some(msg.to_owned());
                    }
                }
                self.promise = None;
            } else {
                ui.spinner();
            }
        }

        if let Some(error) = &self.error_msg {
            ui.label(error);
        }
    }
}
