use eframe::egui::{self, CentralPanel, TextEdit};
use eframe::App as EframeApp;
use bakalari::BakalariClient;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::runtime::Runtime;

#[derive(Serialize, Deserialize, Default, Clone)] // Add Clone trait
struct Credentials {
    username: String,
    password: String,
    server: String,
    remember: bool,
}

pub struct App {
    client: Option<BakalariClient>,
    credentials: Credentials,
    logged_in: bool,
    active_page: String,
}

impl Default for App {
    fn default() -> Self {
        let creds = load_credentials();
        Self {
            client: None,
            credentials: creds,
            logged_in: false,
            active_page: "Login".to_string(),
        }
    }
}

fn load_credentials() -> Credentials {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Bakadesk", "Bakadesk") {
        let config_path = proj_dirs.config_dir().join("credentials.json");
        if let Ok(data) = fs::read_to_string(config_path) {
            if let Ok(creds) = serde_json::from_str(&data) {
                return creds;
            }
        }
    }
    Credentials::default()
}

fn save_credentials(creds: &Credentials) {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Bakadesk", "Bakadesk") {
        let config_path = proj_dirs.config_dir();
        fs::create_dir_all(config_path).ok();
        let file_path = config_path.join("credentials.json");
        if let Ok(data) = serde_json::to_string(creds) {
            fs::write(file_path, data).ok();
        }
    }
}

impl EframeApp for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if self.active_page == "Login" {
                ui.label("Welcome to Bakadesk");

                ui.horizontal(|ui| {
                    ui.label("School Server:");
                    ui.text_edit_singleline(&mut self.credentials.server);
                });

                ui.horizontal(|ui| {
                    ui.label("Username:");
                    ui.text_edit_singleline(&mut self.credentials.username);
                });

                ui.horizontal(|ui| {
                    ui.label("Password:");
                    ui.add(TextEdit::singleline(&mut self.credentials.password).password(true));
                });

                ui.checkbox(&mut self.credentials.remember, "Remember credentials");

                if ui.button("Login").clicked() {
                    let creds = self.credentials.clone();
                    let rt = Runtime::new().unwrap();
                    match rt.block_on(async {
                        let client = BakalariClient::new(
                            &creds.server,
                            &creds.username,
                            &creds.password,
                        )
                        .await;
                        client
                    }) {
                        Ok(client) => {
                            self.client = Some(client);
                            self.logged_in = true;
                            if self.credentials.remember {
                                save_credentials(&self.credentials);
                            }
                            self.active_page = "Main Menu".to_string();
                        }
                        Err(err) => {
                            ui.label(format!("Login failed: {}", err));
                        }
                    }
                }
            } else if self.active_page == "Main Menu" {
                ui.label("Main Menu");
                if ui.button("Komens").clicked() {
                    self.active_page = "Komens".to_string();
                }
                if ui.button("Absence").clicked() {
                    self.active_page = "Absence".to_string();
                }
                if ui.button("Marks").clicked() {
                    self.active_page = "Marks".to_string();
                }
                if ui.button("Semester").clicked() {
                    self.active_page = "Semester".to_string();
                }
                if ui.button("Timetable").clicked() {
                    self.active_page = "Timetable".to_string();
                }
                if ui.button("Homework").clicked() {
                    self.active_page = "Homework".to_string();
                }
            } else {
                ui.label(format!("Page: {}", self.active_page));
                if ui.button("Back").clicked() {
                    self.active_page = "Main Menu".to_string();
                }
            }
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::default();
    let options = eframe::NativeOptions::default();
    eframe::run_native("Bakadesk", options, Box::new(|_| Box::new(app)));
    Ok(())
}



