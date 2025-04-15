use eframe::{egui, App};
use engine::{Fetcher, HttpMethod, HttpVersion};

struct BrowserApp {
    address: String,
    port: String,
    loading: bool,
    response: String,
    fetcher: Fetcher,
    method: HttpMethod,
    version: HttpVersion,
    user_agent: String,
    headers: Vec<(String, String)>,
    new_header_key: String,
    new_header_value: String,
    body: String,
    error: Option<String>,
}

impl Default for BrowserApp {
    fn default() -> Self {
        Self {
            address: String::new(),
            port: "80".to_string(),
            loading: false,
            response: String::new(),
            fetcher: Fetcher::new(),
            method: HttpMethod::Get,
            version: HttpVersion::Http11,
            user_agent: "KnullBrowser/1.0".to_string(),
            headers: Vec::new(),
            new_header_key: String::new(),
            new_header_value: String::new(),
            body: String::new(),
            error: None,
        }
    }
}

impl App for BrowserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Welcome to Knull Browser!");

            // Input for URL
            ui.horizontal(|ui| {
                ui.label("Enter URL:");
                ui.text_edit_singleline(&mut self.address);
            });

            // Input for Port
            ui.horizontal(|ui| {
                ui.label("Port:");
                ui.text_edit_singleline(&mut self.port);
            });

            // HTTP Method Selection
            ui.horizontal(|ui| {
                ui.label("HTTP Method:");
                egui::ComboBox::from_id_source("http_method_selection") // Unique ID for HTTP Method
                    .selected_text(format!("{:?}", self.method))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.method, HttpMethod::Get, "GET");
                        ui.selectable_value(&mut self.method, HttpMethod::Post, "POST");
                        ui.selectable_value(&mut self.method, HttpMethod::Put, "PUT");
                        ui.selectable_value(&mut self.method, HttpMethod::Delete, "DELETE");
                        ui.selectable_value(&mut self.method, HttpMethod::Options, "OPTIONS");
                        ui.selectable_value(&mut self.method, HttpMethod::Trace, "TRACE");
                        ui.selectable_value(&mut self.method, HttpMethod::Head, "HEAD");
                    });
            });

            // HTTP Version Selection
            ui.horizontal(|ui| {
                ui.label("HTTP Version:");
                egui::ComboBox::from_id_source("http_version_selection") // Unique ID for HTTP Version
                    .selected_text(format!("{:?}", self.version))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.version, HttpVersion::Http10, "HTTP/1.0");
                        ui.selectable_value(&mut self.version, HttpVersion::Http11, "HTTP/1.1");
                        ui.selectable_value(&mut self.version, HttpVersion::Http20, "HTTP/2.0");
                    });
            });

            // User-Agent Input
            ui.horizontal(|ui| {
                ui.label("User-Agent:");
                ui.text_edit_singleline(&mut self.user_agent);
            });

            // Headers Input
            ui.collapsing("Headers", |ui| {
                for (key, value) in &mut self.headers {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(key);
                        ui.text_edit_singleline(value);
                    });
                }

                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.new_header_key);
                    ui.text_edit_singleline(&mut self.new_header_value);
                    if ui.button("Add Header").clicked() {
                        if !self.new_header_key.is_empty() && !self.new_header_value.is_empty() {
                            self.headers.push((
                                self.new_header_key.clone(),
                                self.new_header_value.clone(),
                            ));
                            self.new_header_key.clear();
                            self.new_header_value.clear();
                        }
                    }
                });
            });

            // Body Input
            if matches!(self.method, HttpMethod::Post | HttpMethod::Put) {
                ui.label("Body:");
                ui.text_edit_multiline(&mut self.body);
            }

            // Submit Button
            if ui.button("Send Request").clicked() && !self.loading {
                self.loading = true;
                self.error = None;
                self.response.clear();

                let port = self.port.parse::<u16>().unwrap_or(80);

                let request = engine::HttpRequest {
                    method: self.method.clone(),
                    address: format!("{}:{}", self.address, port),
                    version: self.version.clone(),
                    user_agent: self.user_agent.clone(),
                    headers: self.headers.clone(),
                    body: if self.body.is_empty() { None } else { Some(self.body.clone()) },
                };

                match self.fetcher.fetch(request) {
                    Ok(response) => {
                        self.response = response;
                    }
                    Err(err) => {
                        self.error = Some(err);
                    }
                }

                self.loading = false;
            }

            // Loading Spinner
            if self.loading {
                ui.add(egui::Spinner::new());
            }

            // Display Error
            if let Some(error) = &self.error {
                ui.colored_label(egui::Color32::RED, error);
            }

            // Display Response
            if !self.response.is_empty() {
                ui.label("Response:");
                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui| {
                        ui.label(&self.response);
                    });
            }
        });

        // Request repaint while loading
        if self.loading {
            ctx.request_repaint();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Knull Browser",
        options,
        Box::new(|_cc| Ok(Box::new(BrowserApp::default()))),
    )
}


