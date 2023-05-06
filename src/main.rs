use magic_crypt::{new_magic_crypt,MagicCryptTrait, MagicCrypt256};
use eframe::egui;

mod database;
use database::Store;
use types::Secret;
mod types;

fn main() {

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Secret Store",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    );


}

#[derive(Clone)]
pub struct UpdateSecret {
    id: u64,
    secret: Secret,
    show_window: bool,
    user_to_update: String,
    desc_to_update: String,
    pass_to_update: String,
    encrpytion_key: String

}

impl Default for UpdateSecret {
    fn default() -> Self {
        Self {
            id : 0,
            secret: Secret::default(),
            show_window : false,
            user_to_update: String::new(),
            desc_to_update: String::new(),
            pass_to_update: String::new(),
            encrpytion_key: String::new()
        }        
    }
}

impl UpdateSecret {
    pub fn new(id: u64, secret:Secret) -> UpdateSecret{
        UpdateSecret { 
            id: id,
            secret: secret,
            show_window : true,
            user_to_update: String::new(),
            desc_to_update: String::new(),
            pass_to_update: String::new(),
            encrpytion_key: String::new()
        }
    }

    pub fn ui(&mut self, ui:&mut egui::Ui) {
        
        ui.label(format!("Secret ID to be updated : {}", self.secret.get_string_id()));
        let secret_desc_label = ui.label("Secret Description:");
        ui.text_edit_singleline(&mut self.desc_to_update).labelled_by(secret_desc_label.id);
        
        let secret_username_label = ui.label("Secret Username:");
        ui.text_edit_singleline(&mut self.user_to_update).labelled_by(secret_username_label.id);

        let secret_pass_label = ui.label("Secret Password:");
        ui.text_edit_singleline(&mut self.pass_to_update).labelled_by(secret_pass_label.id);

        let button_name = format!("Cancel update for {}",self.secret.get_string_id());

        if ui.button(button_name).clicked() {
            self.show_window = false;
        };

        if ui.button("Submit changes").clicked() {
            let store = Store::connect(String::from("db/test_conn.db")).unwrap();

            if self.encrpytion_key.len() > 0 {
                let updater_encryptor = new_magic_crypt!(self.encrpytion_key.clone(), 256);
                let new_pass = updater_encryptor.encrypt_str_to_base64(self.pass_to_update.clone());
                store.update_secret(self.id as usize, self.user_to_update.clone(), new_pass, self.desc_to_update.clone()).unwrap();

                println!("Secret updated successfully!");

                self.show_window = false;


            }
        }

    }

    pub fn show(&mut self, ui:&mut egui::Ui) {

        if self.show_window == true {
            let window_title = format!("Update Secret {}", self.id);
            egui::Window::new(window_title).show(ui.ctx(), |ui| self.ui(ui));
        }

    }
}

#[derive(Clone)]
pub struct DecryptSecret {
    id: u64,
    encoded_pass: String,
    encryption_key: String,
    decoded_pass: String,
    show_window : bool
}

impl Default for DecryptSecret {
    fn default() -> Self {
        Self {
            id:0,
            encoded_pass:String::new(),
            encryption_key:String::new(),
            decoded_pass:String::new(),
            show_window : true
        }
    }
}

impl DecryptSecret {

    pub fn new(id:u64,encoded_pass:String,encryption_key:String) -> DecryptSecret{
        DecryptSecret {
            id: id,
            encoded_pass:encoded_pass,
            encryption_key: encryption_key,
            decoded_pass: String::new(),
            show_window: true
        }
    }


    fn decrypt_secret(&mut self) {
        if self.encryption_key.len() > 0 {
            let decryptor = new_magic_crypt!(self.encryption_key.clone(), 256);

            let secret_pass = match decryptor.decrypt_base64_to_string(self.encoded_pass.clone()) {
                Ok(pass) => {
                    println!("Successfully decoded password !");
                    pass
                }
                Err(er) => {
                    println!("Caught error : {}", er);
                    String::from("Can't decrypt secret. Provided Key might be invalid!")
                }
            };

            self.decoded_pass = secret_pass;
        }
    }

    pub fn ui(&mut self, ui:&mut egui::Ui) {
        
        if self.encryption_key.len() > 0 {

            self.decrypt_secret();
            ui.label(format!("Secret pass for: {}", self.id));
            ui.text_edit_singleline(&mut self.decoded_pass);

        }
        else { 
            ui.add(egui::Label::new("Encryption key not specified. Can't decrypt."));
        }

        if ui.button("Done").clicked() {
            self.show_window = false;
        }

    }

    pub fn show(&mut self, ui:&mut egui::Ui) {

        if self.show_window == true {
            let window_title = format!("Secret Value for {}", self.id);
            egui::Window::new(window_title).show(ui.ctx(), |ui| self.ui(ui));
        }

    }
}


#[derive(Clone)]
struct MyApp {
    encrpytion_key: String,
    secrets: Vec<Secret>,
    display_create_secret: bool,
    secret_encryptor: MagicCrypt256,
    secret_username: String,
    secret_password: String,
    secret_description: String,
    updatable_secrets: Vec<UpdateSecret>,
    decoded_secrets: Vec<DecryptSecret>
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            encrpytion_key: String::new(),
            secrets : vec![],
            display_create_secret: false,
            secret_encryptor:new_magic_crypt!("",256),
            secret_username:String::new(),
            secret_password:String::new(),
            secret_description:String::new(),
            updatable_secrets: Vec::new(),
            decoded_secrets: Vec::new()
        }

    }

}

impl MyApp {
    fn new(encryption_key: String, secrets:Vec<Secret>) -> MyApp {
        MyApp {
            encrpytion_key: encryption_key,
            secrets: secrets,
            display_create_secret: false,
            secret_encryptor:new_magic_crypt!("",256),
            secret_username:String::new(),
            secret_password:String::new(),
            secret_description:String::new(),
            updatable_secrets: Vec::new(),
            decoded_secrets: Vec::new()
        }
    }

    fn retrieve_secrets(self) -> Vec<Secret> {
        self.secrets.clone()
    }
}


impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                ui.heading("Secret Store");
                ui.add_space(10.0);
                let encryption_key = ui.label("Encryption key: ");
                ui.add(egui::TextEdit::singleline(&mut self.encrpytion_key).password(true)).labelled_by(encryption_key.id);
                ui.add_space(10.0);
    
                if self.encrpytion_key.len() > 0 {
                    self.secret_encryptor = new_magic_crypt!(&self.encrpytion_key, 256);
                }
                
                ui.add_space(10.0);
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
                
                ui.add_space(10.0);
                
                if ui.button("Create Secret").clicked() { 
                    self.display_create_secret = true;
                }
            });


            let secrets = self.secrets.clone();


            if self.display_create_secret {
                egui::Window::new("Create New Secret").show(&ctx, |ui| {

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
                            ui.label(secret.get_description());
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
                            
                                self.updatable_secrets.push(
                                    UpdateSecret {
                                        id:secret.get_id() as u64,
                                        secret:secret.clone(),
                                        show_window: true,
                                        user_to_update: secret.get_username(),
                                        desc_to_update: secret.get_description(),
                                        pass_to_update: secret.get_password(),
                                        encrpytion_key: self.encrpytion_key.clone()
                                    }
                                )
                                
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