use eframe::egui;

mod database;
mod types;
mod ui;
use database::Store;
use types::Secret;
use ui::{DecryptSecret,UpdateSecret,CreateSecret};
use std::rc::Rc;

pub const DB_PATH:&str = "db/secrets.db";

fn main() {

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Secret Store",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    ).unwrap();

}


#[derive(Clone)]
struct MyApp {
    encrpytion_key: String,
    data_store: Rc<Store>,
    secrets: Vec<Secret>,
    create_secret:CreateSecret,
    updatable_secrets: Vec<UpdateSecret>,
    decoded_secrets: Vec<DecryptSecret>
}

impl Default for MyApp {
    fn default() -> Self {
        let store = Store::connect(String::from(DB_PATH)).unwrap();
        store.create_secrets_table().unwrap();

        Self {
            encrpytion_key: String::new(),
            data_store: Rc::new(store),
            secrets : vec![],
            create_secret: CreateSecret::default(),
            updatable_secrets: Vec::new(),
            decoded_secrets: Vec::new()
        }

    }

}

impl MyApp {
    
    fn refresh_secrets(&mut self) {
        let secrets = match self.data_store.fetch_secrets() {
            Ok(secrets) => {
                secrets
            },
            Err(_er) => {
                Vec::new()
            }
        };

        self.secrets = secrets.clone();
    }
}


impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                ui.heading("Secret Store");
                ui.add_space(10.0);
                let encryption_key = ui.label("Encryption key: ");
                ui.add_space(10.0);
                ui.add(egui::TextEdit::singleline(&mut self.encrpytion_key).password(true)).labelled_by(encryption_key.id);
                ui.add_space(10.0);
    
                
                ui.add_space(20.0);
                
                if ui.button("Create Secret").clicked() { 
                    self.create_secret = CreateSecret::new(
                        String::new(),
                        String::new(),
                        String::new(),
                        self.encrpytion_key.clone(),
                        self.data_store.clone()
                    );

                }

                ui.add_space(20.0);

                self.create_secret.show(ui);

                self.refresh_secrets();

            });


            let secrets = self.secrets.clone();

            if secrets.len() > 0 {

                ui.add_space(10.0);

                egui::ScrollArea::new([true,true]).always_show_scroll(true).show(ui, |ui| {
                    
                    egui::Grid::new("secrets_table").num_columns(4).striped(true).spacing([20.0,20.0]).show(ui, |ui| {
                        
                        ui.label("Secret Description");
                        ui.label("Username");
                        ui.label("Password");
                        ui.label("Action");
                        ui.end_row();
    
                        for secret in secrets {
                            ui.label(secret.get_description());
                            ui.label(secret.get_username());
                            ui.label(secret.get_password());
    
                            let delete_button = egui::Button::new("Delete").fill(egui::Color32::DARK_RED);
                            let clicked_delete = ui.add(delete_button).clicked();
        
                            let update_button = egui::Button::new("Update").fill(egui::Color32::DARK_GRAY);
                            let clicked_update = ui.add(update_button).clicked();
    
                            let decrypt_button = egui::Button::new("Decrypt password").fill(egui::Color32::DARK_GREEN);
                            let clicked_decrypt = ui.add(decrypt_button).clicked();
    
                            ui.end_row();
    
                            if clicked_delete {
                                let deletion_id = secret.get_id();
                                
                                self.data_store.delete_secret(deletion_id).unwrap();
    
                                self.refresh_secrets();
                                
                            }
    
                            if clicked_update { 
                            
                                self.updatable_secrets.push(
                                    UpdateSecret::new(
                                        secret.get_id() as u64,
                                        secret.get_username(),
                                        secret.get_description(),
                                        secret.get_password(),
                                        self.encrpytion_key.clone(),
                                        secret.clone(),
                                        self.data_store.clone())
                                );
                                
                            }

    
    
                            if clicked_decrypt {
                                self.decoded_secrets.push(
                                    DecryptSecret::new(
                                        secret.get_id() as u64,
                                        secret.get_password().clone(),
                                        self.encrpytion_key.clone()
                                    ))
                            }
    
                        }
    
                        for updateable_secret in &mut self.updatable_secrets {
                            updateable_secret.show(ui);
                        }

                        self.refresh_secrets();
    
                        for decoded_secret in &mut self.decoded_secrets {
                            decoded_secret.show(ui);
                        }
                    });

                });

            }
                
            else {
                ui.label("No secrets configured yet");
            }
        });
    }
}