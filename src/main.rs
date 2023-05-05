use magic_crypt::{new_magic_crypt,MagicCryptTrait, MagicCrypt256};
use eframe::egui;

mod database;
use database::Store;
use types::Secret;
mod types;

fn main() {
    let mc = new_magic_crypt!("test_key", 256);

    let test_encryption = mc.encrypt_str_to_base64("test");
    println!("{}", test_encryption);

    let test_decryption = mc.decrypt_base64_to_string("oycLsdYp+7go/RbEKna/2Q==").unwrap();
    println!("{}", test_decryption);

    let store = Store::connect(String::from("db/test_conn.db")).unwrap();
    match store.create_secrets_table() {
        Ok(()) => {
            println!("Table created successfully!");
            ()
        },
        Err(er) => {
            println!("Encountered error while creating secrets table : {}", er);
            println!("Error Code : {}", er.code.unwrap());
        }
    };

    let secrets = match store.fetch_secrets() {
        Ok(secrets) => {
            println!("Secrets reading successful!");
            secrets
        },
        Err(er) => {
            println!("Encountered error while fetching secrets : {}", er);
            println!("Error Code : {}", er.code.unwrap());
            Vec::new()
        }
    };

    println!("Len of secrets : {}", secrets.len());

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Secret Store",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    );




}


struct DecryptButton {
    id: u64,
    button: egui::Button
}

impl Default for DecryptButton {
    fn default() -> Self {
        Self {
            id: 0,
            button: egui::Button::new("Decrypt password").fill(egui::Color32::DARK_GREEN)
        }
    }
}


#[derive(Clone)]
struct MyApp {
    encrpytion_key: String,
    secrets: Vec<Secret>,
    display_update_window: bool,
    display_create_secret: bool,
    secret_encryptor: MagicCrypt256,
    secret_username: String,
    secret_password: String,
    secret_description: String,
    decrypt_password: bool
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            encrpytion_key: String::new(),
            secrets : vec![],
            display_update_window: false,
            display_create_secret: false,
            secret_encryptor:new_magic_crypt!("",256),
            secret_username:String::new(),
            secret_password:String::new(),
            secret_description:String::new(),
            decrypt_password: false
        }
    }

}

impl MyApp {
    fn new(encryption_key: String, secrets:Vec<Secret>) -> MyApp {
        MyApp {
            encrpytion_key: encryption_key,
            secrets: secrets,
            display_update_window: false,
            display_create_secret: false,
            secret_encryptor:new_magic_crypt!("",256),
            secret_username:String::new(),
            secret_password:String::new(),
            secret_description:String::new(),
            decrypt_password: false
        }
    }

    fn retrieve_secrets(self) -> Vec<Secret> {
        self.secrets.clone()
    }
}

impl eframe::App for MyApp {


    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Secret Store");
            ui.horizontal(|ui| {
                let encryption_key = ui.label("Encryption key: ");
                ui.text_edit_singleline(&mut self.encrpytion_key)
                    .labelled_by(encryption_key.id);

                if self.encrpytion_key.len() > 0 {
                    self.secret_encryptor = new_magic_crypt!(&self.encrpytion_key, 256);
                }

            });

            if ui.button("Retrieve secrets").clicked() {
                let store = Store::connect(String::from("db/test_conn.db")).unwrap();

                let secrets = match store.fetch_secrets() {
                    Ok(secrets) => {
                        println!("Secrets reading successful!");
                        secrets
                    },
                    Err(er) => {
                        println!("Encountered error while fetching secrets : {}", er);
                        println!("Error Code : {}", er.code.unwrap());
                        Vec::new()
                    }
                };

                self.secrets = secrets.clone();

            } 

            let secrets = self.secrets.clone();

            if ui.button("Create Secret").clicked() { 
                self.display_create_secret = true;
            }

            if self.display_create_secret {
                egui::Window::new("Create New Secret").show(ctx, |ui| {

                    let secret_username_label = ui.label("Input Secret Username: ");
                    ui.text_edit_singleline(&mut self.secret_username).labelled_by(secret_username_label.id);

                    let secret_pass_label = ui.label("Input Secret Password: ");
                    ui.add(egui::TextEdit::singleline(&mut self.secret_password).password(true)).labelled_by(secret_pass_label.id);

                    let secret_desc_label = ui.label("Input Secret Description: ");
                    ui.text_edit_singleline(&mut self.secret_description).labelled_by(secret_desc_label.id);
                    
                    if ui.button("Cancel").clicked() {
                        self.display_create_secret = false;
                    }

                    if ui.button("Submit").clicked() {

                        println!("Len of encryption key : {}", self.encrpytion_key.len());

                        if self.encrpytion_key.len() > 0 {

                            let store = Store::connect(String::from("db/test_conn.db")).unwrap();

                            self.secret_encryptor = new_magic_crypt!(&self.encrpytion_key, 256);

                            let new_secret = Secret::new(
                                0,
                                self.secret_username.clone(),
                                self.secret_encryptor.encrypt_str_to_base64(self.secret_password.clone()),
                                self.secret_description.clone()
                            );
                            
                            ui.label("Adding Secret to Secrets List !");
                            
                            match store.insert_secret(&new_secret) {
                                Ok(()) => {
                                    println!("Secret added!");
                                    ui.label("Secret added !");
                                    let secrets = match store.fetch_secrets() {
                                        Ok(secrets) => {
                                            println!("Secrets reading successful!");
                                            secrets
                                        },
                                        Err(er) => {
                                            println!("Encountered error while fetching secrets : {}", er);
                                            println!("Error Code : {}", er.code.unwrap());
                                            Vec::new()
                                        }
                                    };
                    
                                    self.secrets = secrets.clone();
                                }
                                
                                Err(er) => {
                                    println!("Encountered error while persisting secret : {}", er);
                                    println!("Error Code : {}", er.code.unwrap());
                                    ui.label("Error adding secret!");
                                }
                            };

                        }
                        
                        else {
                            println!("Unable to create Secret, encryption key not available.");
                            ui.label("Unable to create Secret, encryption key not available.");
                        }

                    }
                });
            }


            if secrets.len() > 0 {

                egui::Grid::new("secrets_table").num_columns(4).striped(true).show(ui, |ui| {
                    ui.label("Secret Description");
                    ui.label("Username");
                    ui.label("Password");
                    ui.label("Action");
                    ui.end_row();

                    for secret in secrets {
                        ui.label(secret.get_description());
                        let username = ui.label(secret.get_description());
                        ui.label(secret.get_password());

                        let delete_button = egui::Button::new("Delete").fill(egui::Color32::DARK_RED);
                        let clicked_delete = ui.add(delete_button).clicked();
    
                        let update_button = egui::Button::new("Update").fill(egui::Color32::DARK_GRAY);
                        let clicked_update = ui.add(update_button).clicked();

                        let decrypt_button = egui::Button::new("Decrypt password").fill(egui::Color32::DARK_GREEN);
                        let password_to_decrypt = ui.add(decrypt_button).clicked();

                        ui.end_row();

                        if clicked_delete {
                            let deletion_id = secret.get_id();
                            
                            let store = Store::connect(String::from("db/test_conn.db")).unwrap();
                            store.delete_secret(deletion_id).unwrap();

                            let secrets = match store.fetch_secrets() {
                                Ok(secrets) => {
                                    println!("Secrets reading successful!");
                                    secrets
                                },
                                Err(er) => {
                                    println!("Encountered error while fetching secrets : {}", er);
                                    println!("Error Code : {}", er.code.unwrap());
                                    Vec::new()
                                }
                            };
            
                            self.secrets = secrets.clone();
                            
                        }

                        if clicked_update { 
                            self.display_update_window = true;
                            
                            
                        }

                        if self.display_update_window {
                            let focus_secret = secret.clone();

                            egui::Window::new("Update Secret").show(ctx,|ui| {


                                let focus_id = focus_secret.get_id();
                                println!("{}", focus_id);

                                let secret_id =  focus_secret.get_string_id();
                                
                                ui.label(focus_secret.get_string_id());
                                ui.label(focus_secret.get_description());
                                ui.label(focus_secret.get_username());
                                ui.label(focus_secret.get_password());

                                if ui.button("Abort").clicked() {
                                    self.display_update_window = false;
                                }
                            });
                        }

                        if password_to_decrypt {
                            self.decrypt_password = true
                        }

                        if self.decrypt_password { 
                            egui::Window::new("Secret value").show(ctx, |ui| {

                                if self.encrpytion_key.len() > 0 {

                                    let encoded_pass = secret.get_password();
                                    let decryptor = new_magic_crypt!(self.encrpytion_key.clone(), 256);
                                    println!("encoded Pass: {}", encoded_pass);
                                    println!("Encryptyion Key: {}", self.encrpytion_key.clone());
                                    let secret_pass = match decryptor.decrypt_base64_to_string(encoded_pass) {
                                        Ok(pass) => {
                                            println!("Successfully decoded password !");
                                            pass
                                        }
                                        Err(er) => {
                                            println!("Caught error : {}", er);
                                            String::new()
                                        }
                                    };
                                    //ui.label(format!("Secret pass for: {} - username : {}", secret.get_description(), secret.get_username()));
                                    ui.label(secret_pass);

                                }
                                else { 
                                    ui.add(egui::Label::new("Encryption key not specified. Can't decrypt."));
                                }

                                if ui.button("Done").clicked() {
                                    self.decrypt_password = false;
                                }
                            });
                        }

                    }
                });
            }
            
            else {
                ui.label("No secrets configured yet");
            }

        });
    }

}